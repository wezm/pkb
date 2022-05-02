pub(crate) mod page;
mod sitemap;
pub(crate) mod tag;

use std::convert::Infallible;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

use comrak::plugins::syntect::SyntectAdapter;
use rocket::fairing::{self, AdHoc, Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::http::{Header, Status};
use rocket::request::{FlashMessage, FromRequest, Outcome};
use rocket::response::content::RawHtml;
use rocket::response::Responder;
use rocket::{Build, Data, Request, Response, Rocket};
use rocket::{Catcher, State};
use sentry::types::Dsn;
use time::format_description::FormatItem;
use time::macros::format_description;
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::settings::Settings;
use crate::{web, PkbError};

#[derive(Responder)]
pub(crate) enum CachedHtml {
    #[response(status = 304)]
    NotModified(CacheControl<LastModified<()>>),
    #[response(content_type = "html")]
    Html(CacheControl<LastModified<String>>),
}

pub(crate) struct IfModifiedSince(OffsetDateTime);

pub fn rocket() -> Rocket<Build> {
    let adapter = Arc::new(SyntectAdapter::new("base16-ocean.dark"));

    rocket::build()
        .attach(RequestTimer(None))
        .manage(adapter)
        .mount("/", routes![home, sitemap::robots, sitemap::show])
        .mount("/", page::routes())
        .mount("/", tag::routes())
        .attach(AdHoc::config::<Settings>())
        .attach(init_settings())
        .mount("/public", FileServer::from("public"))
        .register("/", catchers())
}

pub fn catchers() -> Vec<Catcher> {
    catchers![not_found, internal_server_error]
}

#[get("/")]
pub(crate) fn home<'r>(
    settings: &State<Settings>,
    flash: Option<FlashMessage<'r>>,
    adapter: &State<Arc<SyntectAdapter<'_>>>,
    if_modified_since: Option<IfModifiedSince>,
) -> Result<CachedHtml, PkbError> {
    web::page::show("home", settings, flash, adapter, if_modified_since)
}

#[catch(404)]
fn not_found() -> RawHtml<&'static str> {
    const BODY: &str = include_str!("templates/404.html");
    RawHtml(BODY)
}

#[catch(500)]
fn internal_server_error() -> RawHtml<&'static str> {
    const BODY: &str = include_str!("templates/500.html");
    RawHtml(BODY)
}

pub fn init_settings() -> AdHoc {
    AdHoc::try_on_ignite("Init settings", install_sentry)
}

async fn install_sentry(rocket: Rocket<Build>) -> fairing::Result {
    let settings = rocket.state::<Settings>().expect("no settings in state");

    if let Some(dsn) = settings.sentry_dsn.as_deref() {
        match dsn.parse::<Dsn>() {
            Ok(dsn) => {
                let guard = sentry::init((
                    dsn,
                    sentry::ClientOptions {
                        release: sentry::release_name!(),
                        attach_stacktrace: true,
                        ..Default::default()
                    },
                ));
                info!("Sentry initialised");
                Ok(rocket.manage(guard))
            }
            Err(err) => {
                error!("unable to parse Sentry DSN: {}", err);
                Err(rocket)
            }
        }
    } else {
        Ok(rocket)
    }
}

#[derive(Copy, Clone)]
struct RequestTimer(Option<Instant>);

#[rocket::async_trait]
impl Fairing for RequestTimer {
    fn info(&self) -> Info {
        Info {
            name: "Request timer",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        request.local_cache(|| RequestTimer(Some(Instant::now())));
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let start_time = request.local_cache(|| RequestTimer(None));
        if let Some(Some(duration)) = start_time
            .0
            .map(|st| Instant::now().checked_duration_since(st))
        {
            let us = duration.as_micros();
            if us < 1000 {
                response.set_raw_header("X-Response-Time", format!("{} us", us));
            } else {
                let ms = us / 1000;
                response.set_raw_header("X-Response-Time", format!("{} ms", ms));
            }
        }
    }
}

/// Request guard used to retrieve the start time of a request.
#[derive(Copy, Clone)]
pub struct StartTime(pub Instant);

// Allows a route to access the time a request was initiated.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for StartTime {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match *request.local_cache(|| RequestTimer(None)) {
            RequestTimer(Some(time)) => Outcome::Success(StartTime(time)),
            RequestTimer(None) => Outcome::Failure((Status::InternalServerError, ())),
        }
    }
}

const CACHE_TIME: Duration = Duration::from_secs(60);

impl CachedHtml {
    fn html(last_modified: SystemTime, content: String) -> Self {
        CachedHtml::Html(expires_in(
            CACHE_TIME,
            fresh_when(last_modified.max(crate::BUILD_DATE.into()), content),
        ))
    }

    fn not_modified(last_modified: SystemTime) -> Self {
        CachedHtml::NotModified(expires_in(
            CACHE_TIME,
            fresh_when(last_modified.max(crate::BUILD_DATE.into()), ()),
        ))
    }
}

#[derive(Responder)]
pub(crate) struct CacheControl<R> {
    inner: R,
    cache_control: Header<'static>,
}

#[derive(Responder)]
pub(crate) struct LastModified<R> {
    inner: R,
    last_modified: Header<'static>,
}

fn cache_in_varnish<'r, 'o: 'r, R: Responder<'r, 'o>>(
    duration: Duration,
    responder: R,
) -> CacheControl<R> {
    let value = format!("s-maxage={}, public", duration.as_secs());
    let cache_control = Header::new(rocket::http::hyper::header::CACHE_CONTROL.as_str(), value);

    CacheControl {
        inner: responder,
        cache_control,
    }
}

fn expires_in<'r, 'o: 'r, R: Responder<'r, 'o>>(
    duration: Duration,
    responder: R,
) -> CacheControl<R> {
    let value = format!("max-age={}, public", duration.as_secs());
    let cache_control = Header::new(rocket::http::hyper::header::CACHE_CONTROL.as_str(), value);

    CacheControl {
        inner: responder,
        cache_control,
    }
}

// Last-Modified: <day-name>, <day> <month> <year> <hour>:<minute>:<second> GMT
const HTTP_DATE: &[FormatItem] = format_description!(
    "[weekday repr:short], [day] [month repr:short] [year] [hour repr:24]:[minute]:[second] GMT"
);

fn fresh_when<'r, 'o: 'r, R: Responder<'r, 'o>>(
    modified: SystemTime,
    responder: R,
) -> LastModified<R> {
    let value = OffsetDateTime::from(modified).format(HTTP_DATE).unwrap();
    let last_modified = Header::new(rocket::http::hyper::header::LAST_MODIFIED.as_str(), value);
    LastModified {
        inner: responder,
        last_modified,
    }
}

impl IfModifiedSince {
    /// Returns a not modified response if fresh, None otherwise
    fn is_fresh(&self, last_modified: SystemTime) -> Option<CachedHtml> {
        (OffsetDateTime::from(last_modified) <= self.0)
            .then(|| CachedHtml::not_modified(last_modified))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IfModifiedSince {
    type Error = Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("if-modified-since") {
            Some(timestamp) if timestamp.ends_with(" GMT") => {
                PrimitiveDateTime::parse(timestamp, HTTP_DATE)
                    .map(|last_modified| {
                        Outcome::Success(IfModifiedSince(last_modified.assume_utc()))
                    })
                    .unwrap_or_else(|_| Outcome::Forward(()))
            }
            Some(_) | None => Outcome::Forward(()),
        }
    }
}

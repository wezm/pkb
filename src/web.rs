mod page;

use std::borrow::Borrow;
use std::sync::Arc;
use std::time::Instant;

use comrak::plugins::syntect::SyntectAdapter;
use rocket::fairing::{self, AdHoc, Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::request::{FromRequest, Outcome};
use rocket::response::content::RawHtml;
use rocket::{Build, Data, Request, Response, Rocket};
use rocket::{Catcher, Route, State};
use sentry::types::Dsn;

use crate::settings::Settings;

pub fn rocket() -> Rocket<Build> {
    let adapter = Arc::new(SyntectAdapter::new("base16-ocean.dark"));

    rocket::build()
        .attach(RequestTimer(None))
        .manage(adapter)
        .mount("/", page::routes())
        .attach(AdHoc::config::<Settings>())
        .attach(init_settings())
        .mount("/public", FileServer::from("public"))
        .register("/", catchers())
}

pub fn catchers() -> Vec<Catcher> {
    catchers![not_found, internal_server_error]
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

async fn install_sentry(mut rocket: Rocket<Build>) -> fairing::Result {
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

use crate::page::Page;
use rocket::http::uri::Origin;
use rocket::response::content::RawXml;
use rocket::response::Debug;
use rocket::State;
use sitemap::structs::{ChangeFreq, LastMod, Location, Priority, UrlEntry};
use sitemap::writer::SiteMapWriter;
use std::time::{Duration, SystemTime};
use time::{OffsetDateTime, Time};

use crate::settings::Settings;
use crate::tag::Tag;
use crate::web::{self, cache_in_varnish, CacheControl};

const ONE_DAY: Duration = Duration::from_secs(24 * 60 * 60);

struct EntryFactory<'settings> {
    settings: &'settings Settings,
    buf: String,
}

#[get("/robots.txt")]
pub(crate) fn robots(settings: &State<Settings>) -> String {
    format!(
        "User-Agent: *\nDisallow: \nSitemap: https://{}{}\n",
        settings.domain,
        uri!(show)
    )
}

#[get("/sitemap.xml")]
pub(crate) fn show<'r>(
    settings: &State<Settings>,
) -> Result<CacheControl<RawXml<Vec<u8>>>, Debug<sitemap::Error>> {
    // NOTE: Most of the sitemap methods can fail due to I/O errors but since our writer is
    // Vec<u8> we don't expect these in practice. As a result we return Debug<sitemap::Error>>
    // instead of setting up a custom error type that implements Responder.
    let mut sitemap = Vec::new();
    let sitemap_writer = SiteMapWriter::new(&mut sitemap);
    let mut urlwriter = sitemap_writer.start_urlset()?;
    let mut factory = EntryFactory {
        settings: &settings,
        buf: String::new(),
    };

    if let Some(home) = Page::home(&settings.pages_path) {
        let entry = factory.for_page(&home, uri!(web::home), 0.9);
        urlwriter.url(entry)?;

        // /pages
        let entry = factory.for_page(&home, uri!(web::page::index), 0.9);
        urlwriter.url(entry)?;

        // /tags
        let entry = factory.for_page(&home, uri!(web::tag::index), 0.6);
        urlwriter.url(entry)?;
    }

    for page in Page::all(&settings.pages_path) {
        match page.load() {
            Ok(page) if !page.is_hidden() => {
                let entry = factory.for_page(&page, uri!(web::page::show(name = &page.name)), 1.0);
                urlwriter.url(entry)?;
            }
            _ => {}
        }
    }

    for tag in Tag::all(&settings.pages_path) {
        let entry = factory.for_tag(&tag);
        urlwriter.url(entry)?;
    }

    let _ = urlwriter.end()?;

    Ok(cache_in_varnish(ONE_DAY, RawXml(sitemap)))
}

impl<'settings> EntryFactory<'settings> {
    fn for_page<T: std::fmt::Debug>(
        &mut self,
        page: &Page<T>,
        path: Origin<'_>,
        priority: f32,
    ) -> UrlEntry {
        UrlEntry {
            loc: self.loc(path),
            lastmod: self.last_mod(page.last_modified(&self.settings.pages_path)),
            changefreq: ChangeFreq::Weekly,
            priority: Priority::Value(priority),
        }
    }

    fn for_tag(&mut self, tag: &Tag) -> UrlEntry {
        UrlEntry {
            loc: self.loc(uri!(web::tag::show(name = &tag.name))),
            lastmod: self.last_mod(tag.last_modified()),
            changefreq: ChangeFreq::Weekly,
            priority: Priority::Value(0.6),
        }
    }

    fn loc(&mut self, path: Origin<'_>) -> Location {
        self.buf.clear();
        self.buf.push_str("https://");
        self.buf.push_str(&self.settings.domain);
        self.buf.push_str(&path.to_string());
        self.buf.as_str().into()
    }

    fn last_mod(&self, last_modified: SystemTime) -> LastMod {
        let date = OffsetDateTime::from(last_modified);
        // Drop fractional seconds
        LastMod::DateTime(
            date.replace_time(Time::from_hms(date.hour(), date.minute(), date.second()).unwrap()),
        )
    }
}

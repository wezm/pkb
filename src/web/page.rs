use std::sync::Arc;
use std::time::Duration;

use comrak::plugins::syntect::SyntectAdapter;
use rocket::request::FlashMessage;
use rocket::{Route, State};

use crate::page::Page;
use crate::settings::Settings;
use crate::templates::page::{Index, Show};
use crate::templates::{Layout, Nil};
use crate::web::{expires_in, fresh_when, CachedHtml};
use crate::{html, PkbError};

pub fn routes() -> Vec<Route> {
    routes![index, show]
}

#[get("/<name>", rank = 2)]
pub(crate) fn show<'r>(
    name: &'r str,
    settings: &State<Settings>,
    flash: Option<FlashMessage<'r>>,
    adapter: &State<Arc<SyntectAdapter<'_>>>,
) -> Result<CachedHtml, PkbError> {
    let page = Page::new(name, &settings.pages_path)
        .ok_or(PkbError::PageNotFound)?
        .load()?;
    let content = Layout {
        settings,
        title: &page.title(),
        flash: flash.as_ref(),
        head: Nil {},
        body: Show {
            page: &page,
            settings,
            adapter: &*adapter,
        },
    };
    Ok(expires_in(
        Duration::from_secs(60),
        fresh_when(page.last_modified(&settings.pages_path), html(content)),
    ))
}

#[get("/pages")]
pub(crate) fn index<'r>(
    settings: &State<Settings>,
    flash: Option<FlashMessage<'r>>,
) -> Result<CachedHtml, PkbError> {
    let mut pages = Page::all(&settings.pages_path);
    pages.sort_by(|a, b| a.name.cmp(&b.name));
    let pages = pages
        .into_iter()
        .filter_map(|page| page.load().ok())
        .filter(|page| !page.is_hidden())
        .collect::<Vec<_>>();

    let page = Layout {
        settings,
        title: "Index",
        flash: flash.as_ref(),
        head: Nil {},
        body: Index { pages: &pages },
    };
    Ok(expires_in(
        Duration::from_secs(60),
        fresh_when(Page::last_modified_page(&settings.pages_path), html(page)),
    ))
}

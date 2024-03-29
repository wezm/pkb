use std::sync::Arc;

use comrak::plugins::syntect::SyntectAdapter;
use rocket::fs::FileName;
use rocket::{Route, State};

use crate::page::Page;
use crate::settings::Settings;
use crate::templates::page::{Index, Show};
use crate::templates::{Layout, Nil};
use crate::web::{CachedHtml, IfModifiedSince};
use crate::{return_if_fresh, PkbError};

pub fn routes() -> Vec<Route> {
    routes![index, show]
}

#[get("/<name>", rank = 2)]
pub(crate) fn show<'r>(
    name: &'r str,
    settings: &State<Settings>,
    adapter: &State<Arc<SyntectAdapter>>,
    modified_since: Option<IfModifiedSince>,
) -> Result<CachedHtml, PkbError> {
    let page = Page::new(FileName::new(name), &settings.pages_path)
        .ok_or(PkbError::PageNotFound)?
        .load()?;
    return_if_fresh!(modified_since, page.last_modified(&settings.pages_path));

    let content = Layout {
        settings,
        title: &page.title(),
        head: Nil {},
        body: Show {
            page: &page,
            settings,
            adapter: &*adapter,
        },
    };
    Ok(CachedHtml::html(
        page.last_modified(&settings.pages_path),
        content.to_string(),
    ))
}

#[get("/pages")]
pub(crate) fn index<'r>(
    settings: &State<Settings>,
    modified_since: Option<IfModifiedSince>,
) -> Result<CachedHtml, PkbError> {
    let mut pages = Page::all(&settings.pages_path);
    return_if_fresh!(
        modified_since,
        Page::last_modified_page(&settings.pages_path)
    );

    pages.sort_by(|a, b| a.name.cmp(&b.name));
    let pages = pages
        .into_iter()
        .filter_map(|page| page.load().ok())
        .filter(|page| !page.is_hidden())
        .collect::<Vec<_>>();

    let page = Layout {
        settings,
        title: "Index",
        head: Nil {},
        body: Index { pages: &pages },
    };
    Ok(CachedHtml::html(
        Page::last_modified_page(&settings.pages_path),
        page.to_string(),
    ))
}

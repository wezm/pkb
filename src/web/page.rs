use std::sync::Arc;

use comrak::plugins::syntect::SyntectAdapter;
use rocket::request::FlashMessage;
use rocket::response::content::RawHtml;
use rocket::{Route, State};

use crate::page::Page;
use crate::settings::Settings;
use crate::templates::page::{Index, Show};
use crate::templates::{Layout, Nil};
use crate::{html, PkbError};

pub fn routes() -> Vec<Route> {
    routes![index, show]
}

#[get("/<name>", rank = 2)]
pub(crate) async fn show<'r>(
    name: &'r str,
    settings: &State<Settings>,
    flash: Option<FlashMessage<'r>>,
    adapter: &State<Arc<SyntectAdapter<'_>>>,
) -> Result<RawHtml<String>, PkbError> {
    let page = Page::new(name, &settings.pages_path)
        .ok_or(PkbError::PageNotFound)?
        .load()?;
    let page = Layout {
        settings,
        title: &page.title(),
        flash: flash.as_ref(),
        head: Nil {},
        body: Show {
            page: &page,
            adapter: &*adapter,
        },
    };
    Ok(html(page))
}

#[get("/pages")]
pub(crate) async fn index<'r>(
    settings: &State<Settings>,
    flash: Option<FlashMessage<'r>>,
) -> Result<RawHtml<String>, PkbError> {
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
    Ok(html(page))
}

use std::time::Duration;

use rocket::request::FlashMessage;
use rocket::{Route, State};

use crate::page::Page;
use crate::settings::Settings;
use crate::tag::Tag;
use crate::templates::tag::{Index, Show};
use crate::templates::{Layout, Nil};
use crate::web::{expires_in, fresh_when, CachedHtml};
use crate::{html, PkbError};

pub fn routes() -> Vec<Route> {
    routes![index, show]
}

#[get("/tags/<name>")]
pub(crate) fn show<'r>(
    name: &'r str,
    settings: &State<Settings>,
    flash: Option<FlashMessage<'r>>,
) -> Result<CachedHtml, PkbError> {
    let tag = Tag::find(name, &settings.pages_path).ok_or(PkbError::PageNotFound)?;

    let page = Layout {
        settings,
        title: "Tags",
        flash: flash.as_ref(),
        head: Nil {},
        body: Show { tag: &tag },
    };
    Ok(expires_in(
        Duration::from_secs(60),
        fresh_when(tag.last_modified(), html(page)),
    ))
}

#[get("/tags")]
pub(crate) fn index<'r>(
    settings: &State<Settings>,
    flash: Option<FlashMessage<'r>>,
) -> Result<CachedHtml, PkbError> {
    let tags = Tag::all(&settings.pages_path);

    let page = Layout {
        settings,
        title: "Tags",
        flash: flash.as_ref(),
        head: Nil {},
        body: Index { tags: &tags },
    };
    Ok(expires_in(
        Duration::from_secs(60),
        fresh_when(Page::last_modified_page(&settings.pages_path), html(page)),
    ))
}

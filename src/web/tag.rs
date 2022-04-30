use rocket::request::FlashMessage;
use rocket::{Route, State};

use crate::page::Page;
use crate::settings::Settings;
use crate::tag::Tag;
use crate::templates::tag::{Index, Show};
use crate::templates::{Layout, Nil};
use crate::web::CachedHtml;
use crate::PkbError;

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
    Ok(CachedHtml::html(tag.last_modified(), page.to_string()))
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
    Ok(CachedHtml::html(
        Page::last_modified_page(&settings.pages_path),
        page.to_string(),
    ))
}

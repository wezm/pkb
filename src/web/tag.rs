use rocket::{Route, State};

use crate::page::Page;
use crate::settings::Settings;
use crate::tag::Tag;
use crate::templates::tag::{Index, Show};
use crate::templates::{Layout, Nil};
use crate::web::{CachedHtml, IfModifiedSince};
use crate::{return_if_fresh, PkbError};

pub fn routes() -> Vec<Route> {
    routes![index, show]
}

#[get("/tags/<name>")]
pub(crate) fn show<'r>(
    name: &'r str,
    settings: &State<Settings>,
    modified_since: Option<IfModifiedSince>,
) -> Result<CachedHtml, PkbError> {
    let tag = Tag::find(name, &settings.pages_path).ok_or(PkbError::PageNotFound)?;
    return_if_fresh!(modified_since, tag.last_modified());

    let page = Layout {
        settings,
        title: "Tags",
        head: Nil {},
        body: Show { tag: &tag },
    };
    Ok(CachedHtml::html(tag.last_modified(), page.to_string()))
}

#[get("/tags")]
pub(crate) fn index<'r>(
    settings: &State<Settings>,
    modified_since: Option<IfModifiedSince>,
) -> Result<CachedHtml, PkbError> {
    let tags = Tag::all(&settings.pages_path);
    return_if_fresh!(
        modified_since,
        Page::last_modified_page(&settings.pages_path)
    );

    let page = Layout {
        settings,
        title: "Tags",
        head: Nil {},
        body: Index { tags: &tags },
    };
    Ok(CachedHtml::html(
        Page::last_modified_page(&settings.pages_path),
        page.to_string(),
    ))
}

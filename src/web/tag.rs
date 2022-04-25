use rocket::request::FlashMessage;
use rocket::response::content::RawHtml;
use rocket::{Route, State};

use crate::settings::Settings;
use crate::tag::Tag;
use crate::templates::tag::Index;
use crate::templates::{Layout, Nil};
use crate::{html, PkbError};

pub fn routes() -> Vec<Route> {
    routes![index, show]
}

#[get("/tags/<name>")]
pub(crate) async fn show<'r>(
    name: &'r str,
    settings: &State<Settings>,
    flash: Option<FlashMessage<'r>>,
) -> Result<RawHtml<String>, PkbError> {
    todo!()
}

#[get("/tags")]
pub(crate) async fn index<'r>(
    settings: &State<Settings>,
    flash: Option<FlashMessage<'r>>,
) -> Result<RawHtml<String>, PkbError> {
    let tags = Tag::all(&settings.pages_path);

    let page = Layout {
        settings,
        title: "Tags",
        flash: flash.as_ref(),
        head: Nil {},
        body: Index { tags: &tags },
    };
    Ok(html(page))
}

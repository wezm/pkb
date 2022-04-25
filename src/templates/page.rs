use comrak::plugins::syntect::SyntectAdapter;

use crate::page::{Loaded, Page};
use crate::{templates, web};

markup::define! {
    Show<'a>(page: &'a Page<Loaded>, adapter: &'a SyntectAdapter<'a>) {
        @markup::raw(templates::enhance_markup(&templates::markdown(page.markdown(), adapter)))
    }

    Index<'a>(pages: &'a [Page<Loaded>]) {
        h2 { "Index" }

        ul {
            @for page in *pages {
                li { a[href=uri!(web::page::show(name=&page.name)).to_string()] { @page.title() } }
            }
        }
    }
}

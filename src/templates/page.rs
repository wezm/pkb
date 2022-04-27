use comrak::plugins::syntect::SyntectAdapter;

use crate::page::{Loaded, Page};
use crate::settings::Settings;
use crate::templates::decorators::enhance_markup;
use crate::{templates, web};

markup::define! {
    Show<'a>(page: &'a Page<Loaded>, settings: &'a Settings, adapter: &'a SyntectAdapter<'a>) {
        @markup::raw(enhance_markup(&templates::markdown(page.markdown(), adapter), settings))
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

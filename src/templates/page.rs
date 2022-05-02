use comrak::plugins::syntect::SyntectAdapter;

use crate::page::{Loaded, Page};
use crate::settings::Settings;
use crate::templates::decorators::enhance_markup;
use crate::{templates, web};

markup::define! {
    Show<'a>(page: &'a Page<Loaded>, settings: &'a Settings, adapter: &'a SyntectAdapter<'a>) {
        article {
            h1 { a."no-decoration"[href=uri!(web::page::show(name=&page.name)).to_string()] { @page.title() } }

            @markup::raw(enhance_markup(&templates::markdown(page.markdown(), adapter), settings))

            div."smaller-font lighten top-gap-double-em shaded-panel" {
                "Last modified: " abbr[title=page.mtime_rfc3339()] { @page.mtime_date() }
                ul."list-flat list-spaced-right" {
                    @for tag in page.tags() {
                        li { a[href=uri!(web::tag::show(name=tag)).to_string(), rel="tag"] { "#" @tag } }
                    }
                }
            }
        }
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

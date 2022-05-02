use crate::tag::Tag;
use crate::web;

markup::define! {
    Show<'a>(tag: &'a Tag) {
        h1 { @tag.name }

        ul {
            @for page in tag.sorted_pages() {
                li {
                    a[href=uri!(web::page::show(name=&page.name)).to_string()] { @page.title() }
                }
            }
        }
    }

    Index<'a>(tags: &'a [Tag]) {
        h2 { "Tags" }

        ul {
            @for tag in *tags {
                li {
                    a[href=uri!(web::tag::show(name=&tag.name)).to_string()] { @tag.name }
                    " "
                    span.badge.lighten.monospace { @tag.page_count() }
                }
            }
        }
    }
}

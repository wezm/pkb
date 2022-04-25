use crate::tag::Tag;
use crate::web;

markup::define! {
    // Show<'a>(tag: &'a Page<Loaded>, adapter: &'a SyntectAdapter<'a>) {
    //     @markup::raw(templates::markdown(tag.markdown(), adapter))
    // }

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

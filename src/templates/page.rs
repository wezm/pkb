use comrak::plugins::syntect::SyntectAdapter;

use crate::page::{Loaded, Page};
use crate::templates;

markup::define! {
    Show<'a>(page: &'a Page<Loaded>, adapter: &'a SyntectAdapter<'a>) {
        @markup::raw(templates::markdown(page.markdown(), adapter))
    }

    Index<'a>(pages: &'a [Page<Loaded>]) {
        "page index"
    }
}

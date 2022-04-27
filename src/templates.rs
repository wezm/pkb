mod decorators;
mod layout;
pub(crate) mod page;
pub(crate) mod tag;

use comrak::plugins::syntect::SyntectAdapter;

pub use layout::{Layout, Nil};

// Render markdown to HTML
fn markdown(v: &str, adapter: &SyntectAdapter<'_>) -> String {
    use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true; // Allow raw HTML
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(adapter);
    markdown_to_html_with_plugins(v, &options, &plugins)
}

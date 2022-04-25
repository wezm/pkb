mod layout;
pub(crate) mod page;

use comrak::plugins::syntect::SyntectAdapter;

pub use layout::{Layout, Nil};

fn markdown(v: &str, adapter: &SyntectAdapter<'_>) -> String {
    use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
    let options = ComrakOptions::default();
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(adapter);
    markdown_to_html_with_plugins(v, &options, &plugins)
}

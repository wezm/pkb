mod decorators;
mod layout;
pub(crate) mod page;
pub(crate) mod tag;

use comrak::plugins::syntect::SyntectAdapter;

pub use layout::{Layout, Nil};

// Render markdown to HTML
fn markdown(v: &str, adapter: &SyntectAdapter) -> String {
    use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true; // Allow raw HTML
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(adapter);
    markdown_to_html_with_plugins(v, &options, &plugins)
}

#[cfg(test)]
mod tests {
    use super::*;

    // This isn't so much a test but documentation that comrak wraps the custom elements in a <p>
    // tag.
    #[test]
    fn test_custom_elements() {
        let md = "## Recently Updated Pages\n\n<recently-changed-list></recently-changed-list>\n";
        let adapter = SyntectAdapter::new(Some("base16-ocean.dark"));
        let html = markdown(md, &adapter);
        assert_eq!(html, "<h2>Recently Updated Pages</h2>\n<p><recently-changed-list></recently-changed-list></p>\n")
    }
}

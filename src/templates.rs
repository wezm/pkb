mod layout;
pub(crate) mod page;
pub(crate) mod tag;

use comrak::plugins::syntect::SyntectAdapter;
use html5ever::{LocalName, Namespace, QualName};
use kuchiki::traits::TendrilSink;
use kuchiki::{parse_html, Attribute, ExpandedName};

use crate::string_ext::StringExt;
pub use layout::{Layout, Nil};

// Render markdown to HTML
fn markdown(v: &str, adapter: &SyntectAdapter<'_>) -> String {
    use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
    let options = ComrakOptions::default();
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(adapter);
    markdown_to_html_with_plugins(v, &options, &plugins)
}

// Enhance the HTML
fn enhance_markup(html: &str) -> String {
    let mut doc = parse_html().one(html);

    link_headings(&mut doc);
    process_custom_elements(&mut doc);

    doc.to_string()
}

fn link_headings(doc: &kuchiki::NodeRef) {
    for heading in doc.select("h1,h2,h3,h4,h5,h6").unwrap() {
        let identifier = heading.text_contents().to_slug();

        let a = kuchiki::NodeRef::new_element(
            el_name("a"),
            [
                attr("class", String::from("anchor")),
                attr("href", format!("#{}", identifier)),
                attr("id", identifier),
            ],
        );
        let span = kuchiki::NodeRef::new_element(
            el_name("span"),
            [attr("class", String::from("link-icon monospace"))],
        );
        span.append(kuchiki::NodeRef::new_text("#"));
        a.append(span);

        heading.as_node().prepend(a)
    }
}

fn process_custom_elements(doc: &kuchiki::NodeRef) {}

fn attr(name: &str, value: String) -> (ExpandedName, Attribute) {
    (
        ExpandedName::new(Namespace::from(""), LocalName::from(name)),
        Attribute {
            prefix: None,
            value,
        },
    )
}

fn el_name(name: &str) -> QualName {
    QualName::new(None, ns!(html), LocalName::from(name))
}

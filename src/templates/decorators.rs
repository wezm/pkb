use std::path::Path;

use html5ever::{LocalName, Namespace, QualName};
use kuchiki::traits::TendrilSink;
use kuchiki::{parse_fragment, Attribute, ExpandedName, NodeRef};

use crate::page::Page;
use crate::settings::Settings;
use crate::web;

use crate::string_ext::StringExt;

const START_HTML: &str = "<html>";
const END_HTML: &str = "</html>";

// Enhance the HTML
pub fn enhance_markup(html: &str, settings: &Settings) -> String {
    let doc = parse_markup(html);

    link_headings(&doc);
    process_custom_elements(&doc, &settings.pages_path);
    trim_pre_whitespace(&doc);

    let mut enhanced_html = doc.to_string();
    // HACK: The document ends up serialised with a wrapping `<html>` element around the content
    // strip that here. E.g.
    // https://github.com/kuchiki-rs/kuchiki/blob/f652e38b12cb0d33f7bb0565b6933a6e2823a0c5/src/tests.rs#L66
    if enhanced_html.ends_with(END_HTML) {
        enhanced_html.truncate(enhanced_html.len() - END_HTML.len());
    }
    if enhanced_html.starts_with(START_HTML) {
        enhanced_html[START_HTML.len()..].to_string()
    } else {
        enhanced_html
    }
}

fn link_headings(doc: &NodeRef) {
    for heading in doc.select("h1,h2,h3,h4,h5,h6").unwrap() {
        let identifier = heading.text_contents().to_slug();

        let a = NodeRef::new_element(
            el_name("a"),
            [
                attr("class", String::from("anchor")),
                attr("href", format!("#{}", identifier)),
                attr("id", identifier),
            ],
        );
        let span = NodeRef::new_element(
            el_name("span"),
            [attr("class", String::from("link-icon monospace"))],
        );
        span.append(NodeRef::new_text("#"));
        a.append(span);

        heading.as_node().prepend(a)
    }
}

fn process_custom_elements(doc: &NodeRef, basepath: &Path) {
    RecentlyChangedList::process(doc, basepath);
}

fn trim_pre_whitespace(doc: &NodeRef) {
    for code in doc.select("pre code").unwrap() {
        let mut next_node = code.as_node().first_child();
        while let Some(ref node) = next_node {
            let tmp_next = node.next_sibling();
            if node.as_text().is_some() {
                node.detach();
            }
            next_node = tmp_next;
        }
    }
}

struct RecentlyChangedList;

const RECENTLY_MODIFIED_LIMIT: usize = 5;

impl RecentlyChangedList {
    fn process(doc: &NodeRef, basepath: &Path) {
        let p_qual = QualName::new(None, ns!(html), local_name!("p"));
        for elem in doc.select("recently-changed-list").unwrap() {
            // comrak wraps the element in <p> tag so we need to replace it to avoid generating
            // invalid markup
            let parent = elem.as_node().parent();
            let node_to_replace = match parent {
                Some(ref parent) if parent.as_element().map_or(false, |e| e.name == p_qual) => {
                    parent
                }
                Some(_) | None => elem.as_node(),
            };

            let list = NodeRef::new_element(el_name("ul"), []);
            for page in Page::recently_modified(RECENTLY_MODIFIED_LIMIT, basepath) {
                let li = NodeRef::new_element(el_name("li"), []);
                let link = NodeRef::new_element(
                    el_name("a"),
                    [attr(
                        "href",
                        uri!(web::page::show(name = &page.name)).to_string(),
                    )],
                );
                link.append(NodeRef::new_text(page.title()));
                let span = NodeRef::new_element(
                    el_name("span"),
                    [attr("class", String::from("smaller-font lighten"))],
                );
                span.append(NodeRef::new_text(" updated "));
                let abbr =
                    NodeRef::new_element(el_name("abbr"), [attr("title", page.mtime_rfc3339())]);
                abbr.append(NodeRef::new_text(page.mtime_human()));

                span.append(abbr);
                li.append(link);
                li.append(span);
                list.append(li);
            }

            // replace elem with the list
            node_to_replace.insert_after(list);
            node_to_replace.detach();
        }
    }
}

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

fn parse_markup(html: &str) -> NodeRef {
    let ctx_name = QualName::new(None, ns!(html), local_name!("div"));
    info!("{}", html);
    parse_fragment(ctx_name.clone(), vec![]).one(html)
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use rocket::form::validate::Contains;
    use std::path::PathBuf;

    const HTML: &str = "<h1>Test</h1><recently-changed-list></recently-changed-list>";

    fn test_settings() -> Settings {
        Settings {
            pages_path: pages_path(),
            author: "Test".to_string(),
            author_url: "https://example.com/".to_string(),
            copyright_start_year: 2020,
            name: "Test Site".to_string(),
            domain: "example.com".to_string(),
            tagline: "For testing".to_string(),
            sentry_dsn: None,
        }
    }

    fn pages_path() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.extend(&["tests", "fixtures", "pages"]);
        path
    }

    #[test]
    fn recently_changed_list() {
        // Test that it replaces the custom element with a list of pages
        let doc = parse_markup(HTML);
        RecentlyChangedList::process(&doc, &pages_path());

        let processed = doc.to_string();
        let regex = Regex::new(r#"<ul>(<li><a href="/[^"]+">[^<]+</a><span class="smaller-font lighten"> updated <abbr title="\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z">[^<]+</abbr></span></li>)+</ul>"#).unwrap();
        assert!(regex.is_match(&processed));
    }

    #[test]
    fn recently_changed_list_strips_parent_p_tag() {
        // When rendering the Markdown to HTML comrak wraps the custom element in a <p>, test that
        // this is stripped.
        let html = "<h2>Recently Updated Pages</h2>\n<p><recently-changed-list></recently-changed-list></p>\n";
        let doc = parse_markup(html);
        RecentlyChangedList::process(&doc, &pages_path());

        let processed = doc.to_string();
        assert!(!processed.contains("<p>"));
    }

    #[test]
    fn enhancing_markup_does_not_add_html_tag() {
        let markup = "<p>no HTML tag please</p>";
        let enhanced = enhance_markup(markup, &test_settings());
        assert_eq!(markup, enhanced);
    }
}

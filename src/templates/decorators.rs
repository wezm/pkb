use html5ever::{LocalName, Namespace, QualName};
use kuchiki::traits::TendrilSink;
use kuchiki::{parse_html, Attribute, ExpandedName, NodeRef};
use std::path::Path;

use crate::page::Page;
use crate::settings::Settings;
use crate::web;

use crate::string_ext::StringExt;

// Enhance the HTML
pub fn enhance_markup(html: &str, settings: &Settings) -> String {
    let doc = parse_html().one(html);

    link_headings(&doc);
    process_custom_elements(&doc, &settings.pages_path);
    trim_pre_whitespace(&doc);

    doc.to_string()
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
        for elem in doc.select("recently-changed-list").unwrap() {
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
            elem.as_node().insert_after(list);
            elem.as_node().detach();
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

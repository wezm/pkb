use std::collections::BTreeMap;
use std::path::Path;
use std::rc::Rc;
use std::time::SystemTime;

use crate::page::{Loaded, Page};

pub type Pages = Vec<Rc<Page<Loaded>>>;

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pages: Pages,
}

impl Tag {
    fn new(name: String, pages: Pages) -> Tag {
        Tag { name, pages }
    }

    pub(crate) fn all(basepath: &Path) -> Vec<Tag> {
        let mappings = Page::all(basepath)
            .into_iter()
            .filter_map(|page| page.load().ok())
            .filter_map(|page| {
                if page.is_hidden() {
                    None
                } else {
                    Some(Rc::new(page))
                }
            })
            .fold(
                BTreeMap::new(),
                |mut mappings: BTreeMap<String, Pages>, page| {
                    if !page.is_hidden() {
                        page.tags().iter().for_each(|name| {
                            mappings
                                .entry(name.to_string())
                                .or_default()
                                .push(Rc::clone(&page))
                        });
                        mappings
                    } else {
                        mappings
                    }
                },
            );

        mappings
            .into_iter()
            .map(|(name, pages)| Tag::new(name, pages))
            .collect()
    }

    pub fn find(name: &str, basepath: &Path) -> Option<Tag> {
        Tag::all(basepath).into_iter().find(|tag| tag.name == name)
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn sorted_pages(&self) -> Pages {
        let mut sorted = self.pages.clone();
        sorted.sort_by(|a, b| a.name.cmp(&b.name));
        sorted
    }

    pub fn last_modified(&self) -> SystemTime {
        self.pages
            .iter()
            .map(|page| page.mtime())
            .max()
            .unwrap_or_else(|| SystemTime::now())
    }
}

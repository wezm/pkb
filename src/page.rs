use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{fs, io};

use serde::Deserialize;
use time::format_description::well_known::Rfc3339;
use time::format_description::FormatItem;
use time::macros::format_description;
use time::OffsetDateTime;
use titlecase::titlecase;

const YAML_BOUNDARY: &str = "---";

#[derive(Debug)]
pub struct NotLoaded;

#[derive(Debug)]
pub struct Loaded {
    metadata: Metadata,
    content: String,
}

#[derive(Debug)]
pub struct Page<T: Debug> {
    pub name: String,
    path: PathBuf,
    meta: fs::Metadata,
    content: T,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
struct Metadata {
    title: Option<String>,
    tags: Vec<String>,
    hidden: bool,
}

const MTIME_DATE_FORMAT: &[FormatItem] = format_description!("[day] [month repr:long] [year]");
const MTIME_HUMAN_FORMAT: &[FormatItem] =
    format_description!("[day] [month repr:long] [year], [hour repr:12]:[minute] [period] UTC");

impl<T> Page<T>
where
    T: Debug,
{
    pub fn mtime(&self) -> SystemTime {
        self.meta.modified().expect("metadata missing mtime")
    }

    pub fn mtime_human(&self) -> String {
        OffsetDateTime::from(self.mtime())
            .format(MTIME_HUMAN_FORMAT)
            .unwrap()
    }

    pub fn mtime_date(&self) -> String {
        OffsetDateTime::from(self.mtime())
            .format(MTIME_DATE_FORMAT)
            .unwrap()
    }

    pub fn mtime_rfc3339(&self) -> String {
        OffsetDateTime::from(self.mtime()).format(&Rfc3339).unwrap()
    }

    pub fn last_modified(&self, basepath: &Path) -> SystemTime {
        if self.name == "home" {
            // Home page lists recently changed files, so is modified whenever any
            // other page is modified.
            Page::last_modified_page(basepath)
        } else {
            self.mtime()
        }
    }

    fn is_empty(&self) -> bool {
        self.meta.len() == 0
    }

    pub(crate) fn load(self) -> io::Result<Page<Loaded>> {
        let content = fs::read_to_string(&self.path)?;
        let metadata = metadata(&content)?;

        let loaded = Loaded { content, metadata };
        Ok(Page {
            name: self.name,
            path: self.path,
            meta: self.meta,
            content: loaded,
        })
    }
}

impl Page<NotLoaded> {
    pub(crate) fn new(name: &str, basepath: &Path) -> Option<Page<NotLoaded>> {
        let path = basepath.join(name).with_extension("md");
        let meta = fs::metadata(&path).ok()?;
        meta.is_file().then(|| Page {
            name: name.to_string(),
            path,
            meta,
            content: NotLoaded,
        })
    }

    pub(crate) fn all(basepath: &Path) -> Vec<Page<NotLoaded>> {
        Self::page_files_in(basepath)
            .into_iter()
            .filter_map(|path| {
                path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .and_then(|name| Page::new(name, basepath)) // TODO: Log error to create page?
            })
            .filter(|page| !page.is_empty() /*|| page.is_hidden()*/)
            .collect()
    }

    pub(crate) fn recently_modified(limit: usize, basepath: &Path) -> Vec<Page<Loaded>> {
        let mut pages = Self::all(basepath);
        pages.sort_by(|a, b| b.mtime().cmp(&a.mtime()));
        pages
            .into_iter()
            .filter_map(|page| page.load().ok())
            .filter(|page| !page.is_hidden())
            .take(limit)
            .collect()
    }

    fn page_files_in(basepath: &Path) -> Vec<PathBuf> {
        Self::page_file_in_inner(basepath).unwrap_or_else(|_err| {
            // TODO: log error
            Vec::new()
        })
    }

    fn page_file_in_inner(basepath: &Path) -> io::Result<Vec<PathBuf>> {
        let mut pages = Vec::new();
        let md = OsStr::new("md");
        for entry in fs::read_dir(basepath)?.flatten() {
            let file_name = entry.file_name();
            let file_name = Path::new(&file_name);
            if file_name.extension() == Some(md) {
                pages.push(entry.path());
            }
        }
        Ok(pages)
    }

    pub(crate) fn home(basepath: &Path) -> Option<Page<NotLoaded>> {
        Page::new("home", basepath)
    }

    pub fn last_modified_page(basepath: &Path) -> SystemTime {
        Page::all(basepath)
            .iter()
            .map(|page| page.mtime())
            .max()
            .unwrap_or_else(|| SystemTime::now())
    }
}

impl Page<Loaded> {
    pub(crate) fn title(&self) -> String {
        self.metadata()
            .title
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| titlecase(&self.name))
    }

    pub fn markdown(&self) -> &str {
        if self.content().lines().next() == Some(YAML_BOUNDARY) {
            let mut chunks = self.content().splitn(3, YAML_BOUNDARY);
            let _ = chunks.next();
            let _yaml = chunks.next();
            chunks.next().unwrap_or_default()
        } else {
            self.content()
        }
    }

    pub(crate) fn tags(&self) -> &[String] {
        &self.metadata().tags
    }

    pub(crate) fn is_hidden(&self) -> bool {
        self.metadata().hidden
    }

    fn metadata(&self) -> &Metadata {
        &self.content.metadata
    }

    fn content(&self) -> &str {
        &self.content.content
    }
}

fn metadata(content: &str) -> io::Result<Metadata> {
    if content.lines().next() == Some(YAML_BOUNDARY) {
        let mut chunks = content.splitn(3, YAML_BOUNDARY);
        let _ = chunks.next();
        let yaml = chunks
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "missing YAML front-matter"))?;

        serde_yaml::from_str(yaml).map_err(|err| io::Error::new(io::ErrorKind::Other, err))
    } else {
        Ok(Metadata::default())
    }
}

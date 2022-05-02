use std::path::PathBuf;

use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Settings {
    pub pages_path: PathBuf,
    pub author: String,
    pub author_url: String,
    pub copyright_start_year: u16,
    pub name: String,
    pub domain: String,
    pub tagline: String,
    pub sentry_dsn: Option<String>,
}

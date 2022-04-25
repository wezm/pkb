#[macro_use]
extern crate rocket;
#[macro_use]
extern crate html5ever;

use rocket::http::Status;
use rocket::response::{content, Responder};
use rocket::Request;
use std::{fmt, io};

mod page;
mod settings;
pub mod string_ext;
mod tag;
pub(crate) mod templates;
pub mod web;

#[derive(Debug)]
pub enum PkbError {
    Io(io::Error),
    /// Page is invalid or not found
    PageNotFound,
}

impl From<io::Error> for PkbError {
    fn from(err: io::Error) -> Self {
        PkbError::Io(err)
    }
}

impl fmt::Display for PkbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PkbError::Io(err) => err.fmt(f),
            PkbError::PageNotFound => f.write_str("page not found"),
        }
    }
}

impl std::error::Error for PkbError {}

/// Render a template as HTML
pub fn html<T: markup::Render + fmt::Display>(template: T) -> content::RawHtml<String> {
    content::RawHtml(template.to_string())
}

impl<'r> Responder<'r, 'static> for PkbError {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        match self {
            PkbError::PageNotFound => Err(Status::NotFound),
            _ => {
                error!("{}: {}", req.uri(), self);
                sentry::capture_error(&self);
                Err(Status::InternalServerError)
            }
        }
    }
}

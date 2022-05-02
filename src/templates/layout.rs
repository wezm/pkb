use std::ops::RangeInclusive;

use time::OffsetDateTime;

use crate::settings::Settings;
use crate::web;

markup::define! {
    Layout<'a, Head: markup::Render, Body: markup::Render>(title: &'a str, head: Head, body: Body, settings: &'a Settings) {
        @markup::doctype()

        html[lang="en"] {
            head {
                meta[charset="utf-8"];
                meta[name="viewport", content="width=device-width, initial-scale=1"];
                title { @title " - " @settings.name }
                link[rel="stylesheet", href="/public/css/manrope.css", type="text/css"];
                link[rel="stylesheet", href="/public/css/style.css", type="text/css"];
                link[rel="icon", href=r#"data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><text y=".9em" font-size="90">🔗</text></svg>"#];
                @head
            }

            body {
                header."bottom-gap" {
                    form.right[id="search", method="get", action="https://duckduckgo.com/"] {
                        input[type="hidden", name="sites", value=&settings.domain];
                        input[type="hidden", name="kz", value="-1"];
                        input[type="hidden", name="kaf", value="1"];
                        input[type="text", name="q", maxlength="255", placeholder="Search", autocapitalize="off", autocomplete="off", title=format!("Search {}", settings.name) ];
                        " "
                        input[type="submit", value="Search"];
                    }

                    strong."no-margin" { a."no-decoration"[href=uri!(web::home).to_string()] { @settings.name } }
                    nav[class="list-flat list-spaced-left right"] {
                        ul[class="no-margin"] {
                            li { a[href=uri!(web::page::index).to_string()] { "Index" } }
                            li { a[href=uri!(web::tag::index).to_string()] { "Tags" } }
                        }
                    }
                    p."no-margin-top lighten" { @settings.tagline }
                }

                @body

                footer."top-gap smaller-font" {
                    p."center-text" {
                        "Copyright " @markup::raw("&copy;") " "
                            @let years = copyright_years(settings.copyright_start_year);
                        @if years.start() == years.end() {
                            @years.start()
                        }
                        else {
                            @years.start() @markup::raw("&ndash;") @years.end()
                        }
                        " "
                            a[href=&settings.author_url] { @settings.author }
                        br;
                        span."smaller-font center-text" {
                            "Powered by " a[href="https://github.com/wezm/pkb"] { "pkb" }
                        }
                    }
                }
            }
        }
    }

    // An empty renderer for pages that don't have extra head content
    Nil() {}
}

fn copyright_years(start: u16) -> RangeInclusive<u16> {
    // TODO: Probably don't need to get the year on every render
    // perhaps it can be cached
    start..=OffsetDateTime::now_utc().year() as u16
}

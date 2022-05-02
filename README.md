# pkb — Personal Knowledge Base

`pkb` is a small [Rust] application that allows you to efficiently publish a
collection of Markdown files. I run my instance at [linkedlist.org].

[![Build Status](https://api.cirrus-ci.com/github/wezm/pkb.svg)](https://cirrus-ci.com/github/wezm/pkb)

## Configuration

* Copy the `Rocket.sample.toml` file to `Rocket.toml` and fill in your own
  details
* Link the directory with your Markdown files in it. E.g. `ln -s ~/Dropbox/My\ Markdown\ Files pages`
* Start the server, `cargo run` and visit <http://127.0.0.1:8000/pages>
* You should create Markdown file called `home.md`. This file will be shown as
  the homepage: <http://127.0.0.1:8000>

## Deployment

1. Build a release binary: `cargo build --release --locked`, it will be created at
   `target/release/pkb`.
1. Copy the binary, `public` directory, and your `Rocket.toml` to your server (this
   assumes your build machine and server are binary compatible).

## History

`pkb` was originally implemented in 2015 as a Ruby on Rails application. The [ruby branch]
contains the code before it was rewritten in Rust in 2022.

[Rust]: https://www.rust-lang.org/
[linkedlist.org]: https://linkedlist.org/
[ruby branch]: https://github.com/wezm/pkb/tree/ruby

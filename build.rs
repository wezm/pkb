use std::path::PathBuf;
use std::{env, fs};

use time::OffsetDateTime;

fn main() {
    let mut output_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    output_path.push("build_date.rs");

    let now = OffsetDateTime::now_utc();
    let new = format!("pub const BUILD_DATE: time::OffsetDateTime = time::macros::datetime!({} {:02}:{:02} UTC);\n", now.date(), now.hour(), now.minute());

    // Only write if changed
    if fs::read_to_string(&output_path).map_or(true, |existing| existing != new) {
        fs::write(output_path, new).expect("unable to write to build_date.rs");
    }
}

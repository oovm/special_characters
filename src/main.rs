extern crate unic;

pub mod lib;

use std::fmt::format;
use std::fs::File;
use std::io::Write;
use unic::char::range::CharRange;
use unic::ucd::name_aliases_of;
use unic::ucd::Block;
use unic::ucd::NameAliasType;
use lib::xid_start_text;

pub fn save_text(text: String, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

fn main() {
    let s = xid_start_text();
    save_text(s, "doc/xid_start_characters.md").unwrap();
}

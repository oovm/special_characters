extern crate unic;

pub mod lib;

use lib::{xid_continue_text, xid_start_text};
use std::{fs::File, io::Write};

pub fn save_text(text: String, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

fn main() {
    save_text(xid_start_text(), "doc/xid_start_characters.md").unwrap();
    save_text(xid_continue_text(), "doc/xid_continue_characters.md").unwrap();
}

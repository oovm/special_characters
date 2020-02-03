pub mod xid;
pub use self::xid::{XID_Continue_table, XID_Start_table};
pub const UNICODE_VERSION: (u64, u64, u64) = (9, 0, 0);

use unic::{char::range::CharRange, ucd::Block};

pub fn xid_start_text() -> String {
    let table = XID_Start_table.to_vec();
    let mut blocks = vec![];
    let mut groups = vec![];
    let mut group = "Basic Latin";
    let mut chars = String::new();
    let mut name = group;
    for r in table {
        let (s, e) = r;
        let cr = CharRange::closed(s, e).iter().collect::<Vec<_>>();
        name = Block::of(cr[0]).unwrap().name;
        let crs = cr.into_iter().collect::<String>();
        if name == group {
            chars.push_str(&crs);
            chars.push_str("\n")
        }
        else {
            blocks.push(name);
            groups.push(chars);
            group = name;
            chars = crs
        }
    }
    blocks.push(name);
    groups.push(chars);
    let mut text = String::from("# XID Start\n\n");
    for (n, c) in blocks.into_iter().zip(groups.into_iter()) {
        text.push_str(&format!("## {}\n", n));
        text.push_str(&c.trim());
        text.push_str("\n\n");
    }
    return text;
}

pub fn xid_continue_text() -> String {
    let table = XID_Continue_table.to_vec();
    let mut blocks = vec![];
    let mut groups = vec![];
    let mut group = "Basic Latin";
    let mut chars = String::new();
    let mut name = group;
    for r in table {
        let (s, e) = r;
        let cr = CharRange::closed(s, e).iter().collect::<Vec<_>>();
        name = Block::of(cr[0]).unwrap().name;
        let crs = cr.into_iter().collect::<String>();
        if name == group {
            chars.push_str(&crs);
            chars.push_str("\n")
        }
        else {
            blocks.push(name);
            groups.push(chars);
            group = name;
            chars = crs
        }
    }
    blocks.push(name);
    groups.push(chars);
    let mut text = String::from("# XID Start\n\n");
    for (n, c) in blocks.into_iter().zip(groups.into_iter()) {
        text.push_str(&format!("## {}\n", n));
        text.push_str(&c.trim());
        text.push_str("\n\n");
    }
    return text;
}

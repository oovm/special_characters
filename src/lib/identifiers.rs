use crate::ucd::{XID_CONTINUE, XID_START};
use unic::{char::range::CharRange, ucd::Block};

pub fn xid_start_text() -> String {
    let (mut name, mut group) = ("", "Basic Latin");
    let mut chars = String::new();
    let mut text = String::from("# XID Start\n\n");
    let mut count = 0;
    for (s, e) in XID_START {
        name = Block::of(*s).unwrap().name;
        for c in CharRange::closed(*s, *e) {
            chars.push(c);
            count += 1
        }
        chars.push('\n');
        chars.push('\n');
        if name != group {
            text.push_str(&format!("## {}({})\n", group, count));
            text.push_str(&chars);
            text.push('\n');
            count = 0;
            group = name;
            chars = String::new()
        }
    }
    return text;
}

pub fn xid_continue_text() -> String {
    let (mut name, mut group) = ("", "Basic Latin");
    let mut chars = String::new();
    let mut text = String::from("# XID Continue\n\n");
    let mut count = 0;
    for (s, e) in XID_CONTINUE {
        name = Block::of(*s).unwrap().name;
        for c in CharRange::closed(*s, *e) {
            chars.push(c);
            count += 1
        }
        chars.push('\n');
        chars.push('\n');
        if name != group {
            text.push_str(&format!("## {}({})\n", group, count));
            text.push_str(&chars);
            text.push('\n');
            count = 0;
            group = name;
            chars = String::new()
        }
    }
    return text;
}

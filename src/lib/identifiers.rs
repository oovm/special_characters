use crate::ucd::{XID_CONTINUE, XID_START};
use itertools::Itertools;
use unic::{char::range::CharRange, ucd::Block};

pub fn xid_start_text() -> String {
    let mut text = String::from("# XID Start\n\n");
    let blocks = XID_START
        .iter()
        .map(|(s, e)| (Block::of(*s).unwrap().name, CharRange::closed(*s, *e).iter().collect_vec()))
        .into_group_map();
    for (group, v) in blocks.iter().sorted_by_key(|(_, v)| v[0][0]) {
        let count: usize = v.iter().map(|cr| cr.len()).sum();
        let chars = v.iter().map(|cr| cr.iter().join("")).join("\n\n");
        text.push_str(&format!("## {}({})\n\n", group, count));
        text.push_str(&chars);
        text.push_str("\n\n");
    }
    return text;
}

pub fn xid_continue_text() -> String {
    let mut text = String::from("# XID Continue\n\n");
    let blocks = XID_CONTINUE
        .iter()
        .map(|(s, e)| (Block::of(*s).unwrap().name, CharRange::closed(*s, *e).iter().collect_vec()))
        .into_group_map();
    for (group, v) in blocks.iter().sorted_by_key(|(_, v)| v[0][0]) {
        let count: usize = v.iter().map(|cr| cr.len()).sum();
        let chars = v.iter().map(|cr| cr.iter().join("")).join("\n\n");
        text.push_str(&format!("## {}({})\n\n", group, count));
        text.push_str(&chars);
        text.push_str("\n\n");
    }
    return text;
}

use crate::ucd::{XID_CONTINUE, XID_START};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{hash_map::RandomState, BTreeSet, HashMap};
use unic::{char::range::CharRange, ucd::Block};

lazy_static! {
    static ref XID_START_MAP: HashMap<&'static str, Vec<Vec<char>>, RandomState> = XID_START
        .iter()
        .map(|(s, e)| (Block::of(*s).unwrap().name, CharRange::closed(*s, *e).iter().collect_vec()))
        .into_group_map();
    static ref XID_CONTINUE_MAP: HashMap<&'static str, Vec<Vec<char>>, RandomState> = XID_CONTINUE
        .iter()
        .map(|(s, e)| (Block::of(*s).unwrap().name, CharRange::closed(*s, *e).iter().collect_vec()))
        .into_group_map();
}

pub fn xid_start_text() -> String {
    let mut text = String::from("# XID Start\n\n");
    for (group, v) in XID_START_MAP.iter().sorted_by_key(|(_, v)| v[0][0]) {
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
    for (group, v) in XID_CONTINUE_MAP.iter().sorted_by_key(|(_, v)| v[0][0]) {
        let count: usize = v.iter().map(|cr| cr.len()).sum();
        let chars = v.iter().map(|cr| cr.iter().join("")).join("\n\n");
        text.push_str(&format!("## {}({})\n\n", group, count));
        text.push_str(&chars);
        text.push_str("\n\n");
    }
    return text;
}

pub fn xid_diff_text() -> String {
    let mut text = String::from("# XID Difference\n\n");
    for (group, va) in XID_CONTINUE_MAP.iter().sorted_by_key(|(_, v)| v[0][0]) {
        let a: BTreeSet<&char> = va.into_iter().flatten().collect();
        let c = match XID_START_MAP.get(group) {
            None => a,
            Some(vb) => {
                let b: BTreeSet<&char> = vb.into_iter().flatten().collect();
                a.difference(&b).cloned().collect()
            }
        };
        text.push_str(&format!("## {}({})\n\n", group, c.len()));
        text.push_str(&c.iter().join(""));
        text.push_str("\n\n");
    }
    return text;
}

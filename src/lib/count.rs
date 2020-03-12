use crate::ucd::{ID_CONTINUE, ID_START, XID_CONTINUE, XID_START};
use unic::char::range::CharRange;

fn count_set(set: &[(char, char)]) -> usize {
    let mut c = 0;
    for (s, e) in set {
        c += CharRange::closed(*s, *e).iter().count()
    }
    return c;
}

pub fn count_set_all() -> String {
    let mut text = String::from("## Nyar Special Characters\n\n");

    text.push_str(&format!("- ID_START: {}\n", count_set(ID_START)));
    text.push_str(&format!("- ID_CONTINUE: {}\n", count_set(ID_CONTINUE)));
    text.push_str(&format!("- XID_START: {}\n", count_set(XID_START)));
    text.push_str(&format!("- XID_CONTINUE: {}\n", count_set(XID_CONTINUE)));
    return text;
}

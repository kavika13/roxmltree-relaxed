extern crate afl;
extern crate roxmltree_relaxed;

use std::str;

use afl::fuzz;

fn main() {
    fuzz(|data| {
        if let Ok(text) = str::from_utf8(data) {
            let _ = roxmltree_relaxed::Document::parse(&text);
        }
    });
}

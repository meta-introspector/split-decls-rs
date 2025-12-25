use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn to_snake_case<F, I>(mut s: &str, change_case: F) -> String
where
    F: Fn(char) -> I,
    I: Iterator<Item = char>,
{
    let mut words = vec![];
    s = s
        .trim_start_matches(|c: char| {
            if c == '_' {
                words.push(String::new());
                true
            } else {
                false
            }
        });
    for s in s.split('_') {
        let mut last_upper = false;
        let mut buf = String::new();
        if s.is_empty() {
            continue;
        }
        for ch in s.chars() {
            if !buf.is_empty() && buf != "'" && ch.is_uppercase() && !last_upper {
                words.push(buf);
                buf = String::new();
            }
            last_upper = ch.is_uppercase();
            buf.extend(change_case(ch));
        }
        words.push(buf);
    }
    words.join("_")
}

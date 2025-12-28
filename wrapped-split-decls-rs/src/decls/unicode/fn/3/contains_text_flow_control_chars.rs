use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] pub fn contains_text_flow_control_chars (s : & str) -> bool { let mut bytes = s . as_bytes () ; loop { match memchr :: memchr (0xE2 , bytes) { Some (idx) => { let ch = & bytes [idx .. idx + 3] ; match ch { [_ , 0x80 , 0xAA ..= 0xAE] | [_ , 0x81 , 0xA6 ..= 0xA9] => break true , _ => { } } bytes = & bytes [idx + 3 ..] ; } None => { break false ; } } } }
}
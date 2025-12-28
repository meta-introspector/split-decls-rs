use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Extract address from objdump line"] fn extract_address (line : & str) -> Option < String > { let parts : Vec < & str > = line . split_whitespace () . collect () ; if parts . len () > 0 { let addr = parts [0] ; if addr . len () >= 8 && addr . chars () . all (| c | c . is_ascii_hexdigit ()) { return Some (format ! ("0x{}" , addr)) ; } } None }
}
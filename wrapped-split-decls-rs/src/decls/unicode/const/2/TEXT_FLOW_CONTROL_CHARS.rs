use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const TEXT_FLOW_CONTROL_CHARS : & [char] = & ['\u{202A}' , '\u{202B}' , '\u{202D}' , '\u{202E}' , '\u{2066}' , '\u{2067}' , '\u{2068}' , '\u{202C}' , '\u{2069}' ,] ;
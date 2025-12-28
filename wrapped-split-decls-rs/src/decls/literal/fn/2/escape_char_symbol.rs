use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn escape_char_symbol (ch : char) -> Symbol { let s : String = ch . escape_default () . map (Into :: < char > :: into) . collect () ; Symbol :: intern (& s) }
}
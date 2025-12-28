use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Escapes all regular expression meta characters in `text`."] # [doc = ""] # [doc = " The string returned may be safely used as a literal in a regular"] # [doc = " expression."] pub fn escape (text : & str) -> String { let mut quoted = String :: new () ; escape_into (text , & mut quoted) ; quoted }
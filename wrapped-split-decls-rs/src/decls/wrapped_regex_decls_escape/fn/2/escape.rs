use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Escapes all regular expression meta characters in `pattern`."] # [doc = ""] # [doc = " The string returned may be safely used as a literal in a regular"] # [doc = " expression."] pub fn escape (pattern : & str) -> alloc :: string :: String { regex_syntax :: escape (pattern) }
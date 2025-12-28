use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn strip_underscores (symbol : Symbol) -> Symbol { let s = symbol . as_str () ; if s . contains ('_') { let mut s = s . to_string () ; s . retain (| c | c != '_') ; return Symbol :: intern (& s) ; } symbol }
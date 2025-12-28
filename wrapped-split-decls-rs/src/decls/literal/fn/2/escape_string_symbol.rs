use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn escape_string_symbol (symbol : Symbol) -> Symbol { let s = symbol . as_str () ; let escaped = s . escape_default () . to_string () ; if s == escaped { symbol } else { Symbol :: intern (& escaped) } }
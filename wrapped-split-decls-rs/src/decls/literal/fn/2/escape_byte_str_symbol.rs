use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn escape_byte_str_symbol (bytes : & [u8]) -> Symbol { let s = bytes . escape_ascii () . to_string () ; Symbol :: intern (& s) }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Match { pub fn matched_text (& self) -> String { self . matched_node . text () . to_string () } }
}
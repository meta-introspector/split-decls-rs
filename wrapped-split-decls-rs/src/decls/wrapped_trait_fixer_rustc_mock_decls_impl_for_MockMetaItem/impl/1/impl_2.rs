use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MockMetaItem { pub fn has_name (self , _symbol : Symbol) -> bool { true } }
}
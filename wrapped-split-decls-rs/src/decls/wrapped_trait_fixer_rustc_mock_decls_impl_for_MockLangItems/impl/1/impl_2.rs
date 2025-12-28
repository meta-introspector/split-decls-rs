use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl MockLangItems { pub fn clone_trait (self) -> Option < DefId > { Some (DefId) } }
}
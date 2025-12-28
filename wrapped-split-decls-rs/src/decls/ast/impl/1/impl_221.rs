use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl VisibilityKind { pub fn is_pub (& self) -> bool { matches ! (self , VisibilityKind :: Public) } }
}
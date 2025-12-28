use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl From < Box < str > > for SmolStr { # [inline] fn from (s : Box < str >) -> SmolStr { SmolStr :: new (s) } }
}
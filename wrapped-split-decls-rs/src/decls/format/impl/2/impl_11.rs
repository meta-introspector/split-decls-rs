use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl FormatArgumentKind { pub fn ident (& self) -> Option < Ident > { match self { & Self :: Normal => None , & Self :: Named (id) => Some (id) , & Self :: Captured (id) => Some (id) , } } }
}
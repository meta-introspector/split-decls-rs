use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl HasAttrs for ExpandedItem { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = false ; fn attrs (& self) -> & [Attribute] { self . 0 . attrs . as_slice () } fn visit_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) { f (& mut self . 0 . attrs) ; } }
}
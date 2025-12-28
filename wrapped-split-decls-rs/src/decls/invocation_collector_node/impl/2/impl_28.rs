use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl HasAttrs for ExpandedPat { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = false ; fn attrs (& self) -> & [Attribute] { & [] } fn visit_attrs (& mut self , _f : impl FnOnce (& mut AttrVec)) { } }
}
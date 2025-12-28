use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl HasAttrs for ExpandedTy { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = false ; fn attrs (& self) -> & [Attribute] { & [] } fn visit_attrs (& mut self , _f : impl FnOnce (& mut AttrVec)) { } }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl HasAttrs for Stmt { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = StmtKind :: SUPPORTS_CUSTOM_INNER_ATTRS ; fn attrs (& self) -> & [Attribute] { self . kind . attrs () } fn visit_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) { self . kind . visit_attrs (f) ; } }
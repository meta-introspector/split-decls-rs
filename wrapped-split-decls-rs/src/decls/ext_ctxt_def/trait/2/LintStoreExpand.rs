use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub trait LintStoreExpand { fn pre_expansion_lint (& self , sess : & Session , features : & Features , registered_tools : & RegisteredTools , node_id : NodeId , attrs : & [Attribute] , items : & [Box < Item >] , name : Symbol ,) ; }
}
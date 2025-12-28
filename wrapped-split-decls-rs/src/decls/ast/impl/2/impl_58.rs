use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl WhereClause { pub fn is_empty (& self) -> bool { ! self . has_where_token && self . predicates . is_empty () } }
}
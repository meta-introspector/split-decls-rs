use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialOrd for Value { fn partial_cmp (& self , rhs : & Self) -> Option < Ordering > { Some (self . cmp (rhs)) } }
}
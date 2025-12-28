use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq for ExternAbi { fn eq (& self , rhs : & Self) -> bool { self . cmp (rhs) == Ordering :: Equal } }
}
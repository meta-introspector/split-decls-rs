use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn contains_name (attrs : & [impl AttributeExt] , name : Symbol) -> bool { find_by_name (attrs , name) . is_some () }
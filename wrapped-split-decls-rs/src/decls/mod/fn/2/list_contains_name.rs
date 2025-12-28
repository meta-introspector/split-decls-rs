use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn list_contains_name (items : & [MetaItemInner] , name : Symbol) -> bool { items . iter () . any (| item | item . has_name (name)) }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < const MAX_SIZE : usize > AsRef < ObjectIdentifierRef > for ObjectIdentifier < MAX_SIZE > { fn as_ref (& self) -> & ObjectIdentifierRef { self . as_oid_ref () } }
}
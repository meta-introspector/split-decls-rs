use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < const MAX_SIZE : usize > Deref for ObjectIdentifier < MAX_SIZE > { type Target = ObjectIdentifierRef ; fn deref (& self) -> & ObjectIdentifierRef { self . as_oid_ref () } }
}
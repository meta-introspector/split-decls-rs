use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < const MAX_SIZE : usize > Borrow < ObjectIdentifierRef > for ObjectIdentifier < MAX_SIZE > { fn borrow (& self) -> & ObjectIdentifierRef { self . as_oid_ref () } }
}
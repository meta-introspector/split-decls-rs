use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < const MAX_SIZE : usize > PartialEq < ObjectIdentifier < MAX_SIZE > > for ObjectIdentifierRef { fn eq (& self , other : & ObjectIdentifier < MAX_SIZE >) -> bool { self . as_bytes () . eq (other . as_bytes ()) } }
}
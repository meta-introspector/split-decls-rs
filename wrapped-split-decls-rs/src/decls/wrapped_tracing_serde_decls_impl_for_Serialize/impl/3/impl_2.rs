use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Serialize for SerializeRecord < '_ > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let serializer = serializer . serialize_map (None) ? ; let mut visitor = SerdeMapVisitor :: new (serializer) ; self . 0 . record (& mut visitor) ; visitor . finish () } }
}
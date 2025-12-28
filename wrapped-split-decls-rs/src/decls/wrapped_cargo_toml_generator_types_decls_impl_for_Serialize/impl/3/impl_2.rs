use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Serialize for Dependency { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { match self { Dependency :: Version (s) if s . is_empty () => serializer . serialize_str ("*") , Dependency :: Version (s) => serializer . serialize_str (s) , Dependency :: Table (table) => table . serialize (serializer) , } } }
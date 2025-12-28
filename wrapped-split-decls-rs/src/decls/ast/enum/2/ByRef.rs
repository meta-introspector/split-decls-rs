use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Copy , Debug , Eq , PartialEq)] # [derive (Encodable , Decodable , HashStable_Generic , Walkable)] pub enum ByRef { Yes (Mutability) , No , }
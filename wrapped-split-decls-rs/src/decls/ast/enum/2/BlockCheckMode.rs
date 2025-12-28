use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , PartialEq , Encodable , Decodable , Debug , Copy , Walkable)] pub enum BlockCheckMode { Default , Unsafe (UnsafeSource) , }
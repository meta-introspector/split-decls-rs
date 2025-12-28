use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Is the trait definition an auto trait?"] # [derive (Copy , Clone , PartialEq , Encodable , Decodable , Debug , HashStable_Generic , Walkable)] pub enum IsAuto { Yes , No , }
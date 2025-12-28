use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , PartialEq , Eq , Hash , Encodable , Decodable , Debug)] # [derive (HashStable_Generic , Walkable)] pub enum Const { Yes (Span) , No , }
}
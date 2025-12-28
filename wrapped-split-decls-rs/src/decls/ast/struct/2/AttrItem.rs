use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct AttrItem { pub unsafety : Safety , pub path : Path , pub args : AttrArgs , pub tokens : Option < LazyAttrTokenStream > , }
}
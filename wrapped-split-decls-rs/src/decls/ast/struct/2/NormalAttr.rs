use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct NormalAttr { pub item : AttrItem , pub tokens : Option < LazyAttrTokenStream > , }
}
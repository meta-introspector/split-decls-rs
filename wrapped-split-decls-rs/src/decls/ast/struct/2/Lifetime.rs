use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A \"Lifetime\" is an annotation of the scope in which variable"] # [doc = " can be used, e.g. `'a` in `&'a i32`."] # [derive (Clone , Encodable , Decodable , Copy , PartialEq , Eq , Hash , Walkable)] pub struct Lifetime { pub id : NodeId , pub ident : Ident , }
}
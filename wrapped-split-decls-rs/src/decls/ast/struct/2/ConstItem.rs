use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct ConstItem { pub defaultness : Defaultness , pub ident : Ident , pub generics : Generics , pub ty : Box < Ty > , pub expr : Option < Box < Expr > > , pub define_opaque : Option < ThinVec < (NodeId , Path) > > , }
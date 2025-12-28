use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct StaticItem { pub ident : Ident , pub ty : Box < Ty > , pub safety : Safety , pub mutability : Mutability , pub expr : Option < Box < Expr > > , pub define_opaque : Option < ThinVec < (NodeId , Path) > > , }
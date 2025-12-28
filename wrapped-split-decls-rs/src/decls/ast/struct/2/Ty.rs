use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Encodable , Decodable , Debug , Walkable)] pub struct Ty { pub id : NodeId , pub kind : TyKind , pub span : Span , pub tokens : Option < LazyAttrTokenStream > , }
}
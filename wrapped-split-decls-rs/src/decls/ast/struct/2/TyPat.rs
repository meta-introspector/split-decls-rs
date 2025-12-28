use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A pattern type pattern."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct TyPat { pub id : NodeId , pub kind : TyPatKind , pub span : Span , pub tokens : Option < LazyAttrTokenStream > , }
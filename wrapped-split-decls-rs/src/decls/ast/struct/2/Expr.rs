use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " An expression."] # [derive (Clone , Encodable , Decodable , Debug)] pub struct Expr { pub id : NodeId , pub kind : ExprKind , pub span : Span , pub attrs : AttrVec , pub tokens : Option < LazyAttrTokenStream > , }
}
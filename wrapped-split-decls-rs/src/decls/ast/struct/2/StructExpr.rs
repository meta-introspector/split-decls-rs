use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct StructExpr { pub qself : Option < Box < QSelf > > , pub path : Path , pub fields : ThinVec < ExprField > , pub rest : StructRest , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct FormatArgument { pub kind : FormatArgumentKind , pub expr : Box < Expr > , }
}
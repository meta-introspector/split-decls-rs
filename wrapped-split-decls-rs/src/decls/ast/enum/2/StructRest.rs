use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum StructRest { # [doc = " `..x`."] Base (Box < Expr >) , # [doc = " `..`."] Rest (Span) , # [doc = " No trailing `..` or expression."] None , }
}
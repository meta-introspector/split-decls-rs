use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum RangeSyntax { # [doc = " `...`"] DotDotDot , # [doc = " `..=`"] DotDotEq , }
}
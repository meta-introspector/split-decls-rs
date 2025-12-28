use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum RangeSyntax { # [doc = " `...`"] DotDotDot , # [doc = " `..=`"] DotDotEq , }
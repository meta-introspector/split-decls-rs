use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Whether enclosing parentheses are present or not."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum Parens { Yes , No , }
}
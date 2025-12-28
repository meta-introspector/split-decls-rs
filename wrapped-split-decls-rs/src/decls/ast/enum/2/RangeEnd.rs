use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum RangeEnd { # [doc = " `..=` or `...`"] Included (RangeSyntax) , # [doc = " `..`"] Excluded , }
}
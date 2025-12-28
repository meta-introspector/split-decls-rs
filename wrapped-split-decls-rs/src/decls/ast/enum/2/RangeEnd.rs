use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum RangeEnd { # [doc = " `..=` or `...`"] Included (RangeSyntax) , # [doc = " `..`"] Excluded , }
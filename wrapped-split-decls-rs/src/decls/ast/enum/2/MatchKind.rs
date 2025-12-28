use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " The kind of match expression"] # [derive (Clone , Copy , Encodable , Decodable , Debug , PartialEq , Walkable)] pub enum MatchKind { # [doc = " match expr { ... }"] Prefix , # [doc = " expr.match { ... }"] Postfix , }
}
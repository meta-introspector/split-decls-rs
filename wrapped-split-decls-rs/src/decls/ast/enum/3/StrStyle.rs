use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Encodable , Decodable , Debug , Copy , Hash , Eq , PartialEq)] # [derive (HashStable_Generic , Walkable)] pub enum StrStyle { # [doc = " A regular string, like `\"foo\"`."] Cooked , # [doc = " A raw string, like `r##\"foo\"##`."] # [doc = ""] # [doc = " The value is the number of `#` symbols used."] Raw (u8) , }
}
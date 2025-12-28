use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , Encodable , Decodable , Debug , PartialEq , Eq)] pub enum FormatSign { # [doc = " The `+` flag."] Plus , # [doc = " The `-` flag."] Minus , }
}
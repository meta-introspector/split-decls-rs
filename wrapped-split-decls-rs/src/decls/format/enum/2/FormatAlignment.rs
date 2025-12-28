use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , Encodable , Decodable , Debug , PartialEq , Eq)] pub enum FormatAlignment { # [doc = " `{:<}`"] Left , # [doc = " `{:>}`"] Right , # [doc = " `{:^}`"] Center , }
}
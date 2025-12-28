use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Encodable , Decodable , Debug , PartialEq , Eq)] pub enum FormatAlignment { # [doc = " `{:<}`"] Left , # [doc = " `{:>}`"] Right , # [doc = " `{:^}`"] Center , }
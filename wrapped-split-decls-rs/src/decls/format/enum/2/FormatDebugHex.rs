use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Encodable , Decodable , Debug , PartialEq , Eq)] pub enum FormatDebugHex { # [doc = " The `x` flag in `{:x?}`."] Lower , # [doc = " The `X` flag in `{:X?}`."] Upper , }
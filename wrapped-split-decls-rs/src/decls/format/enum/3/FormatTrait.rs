use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Encodable , Decodable , Debug , PartialEq , Eq , Hash)] pub enum FormatTrait { # [doc = " `{}`"] Display , # [doc = " `{:?}`"] Debug , # [doc = " `{:e}`"] LowerExp , # [doc = " `{:E}`"] UpperExp , # [doc = " `{:o}`"] Octal , # [doc = " `{:p}`"] Pointer , # [doc = " `{:b}`"] Binary , # [doc = " `{:x}`"] LowerHex , # [doc = " `{:X}`"] UpperHex , }
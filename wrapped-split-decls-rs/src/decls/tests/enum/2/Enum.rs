use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (PartialEq , Clone , Debug , Encodable_NoContext , Decodable_NoContext)] enum Enum { Variant1 , Variant2 (usize , u32) , Variant3 { a : i32 , b : char , c : bool } , }
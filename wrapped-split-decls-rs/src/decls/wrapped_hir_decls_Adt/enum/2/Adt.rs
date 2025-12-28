use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " A Data Type"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum Adt { Struct (Struct) , Union (Union) , Enum (Enum) , }
}
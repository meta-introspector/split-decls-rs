use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum VariantDef { Struct (Struct) , Union (Union) , Variant (Variant) , }
}
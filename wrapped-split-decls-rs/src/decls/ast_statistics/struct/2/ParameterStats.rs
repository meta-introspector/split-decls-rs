use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default , Serialize , Deserialize)] pub struct ParameterStats { pub count : u64 , pub type_name : String , pub positions : Vec < usize > , pub associated_functions : Vec < String > , }
}
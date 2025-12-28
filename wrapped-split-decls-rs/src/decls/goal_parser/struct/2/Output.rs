use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct Output { pub name : String , # [serde (rename = "type")] pub output_type : String , pub description : String , }
}
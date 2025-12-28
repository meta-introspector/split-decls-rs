use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct SequenceOperation { # [serde (rename = "type")] pub op_type : String , pub tasks : Vec < Task > , }
}
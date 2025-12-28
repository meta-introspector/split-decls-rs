use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize , Deserialize)] pub struct ProbeTemplate { pub node_type : AstNodeType , pub action_template : String , pub wrapper_function : Option < String > , pub priority : u32 , }
}
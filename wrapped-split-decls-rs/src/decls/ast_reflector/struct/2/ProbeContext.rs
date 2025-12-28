use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub struct ProbeContext { pub file_path : PathBuf , pub node_name : String , pub node_type : AstNodeType , pub attributes : Vec < String > , pub visibility : String , pub complexity : f64 , }
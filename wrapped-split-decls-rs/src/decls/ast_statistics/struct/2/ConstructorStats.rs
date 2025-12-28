use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Default , Serialize , Deserialize)] pub struct ConstructorStats { pub count : u64 , pub parameter_types : Vec < String > , pub return_type : String , pub usage_contexts : Vec < String > , }
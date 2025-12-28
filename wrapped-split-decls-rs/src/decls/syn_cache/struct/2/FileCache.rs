use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub struct FileCache { pub content : String , pub modified_time : u64 , pub parsed_ast : Option < String > , }
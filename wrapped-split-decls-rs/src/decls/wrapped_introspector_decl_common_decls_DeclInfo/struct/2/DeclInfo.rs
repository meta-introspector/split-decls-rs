use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub struct DeclInfo { pub node_type : & 'static str , pub name : & 'static str , pub visibility : & 'static str , pub module : & 'static str , pub file : & 'static str , pub line : u32 , pub hash : & 'static str , }
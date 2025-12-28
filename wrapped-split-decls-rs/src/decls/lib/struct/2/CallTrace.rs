use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub struct CallTrace { pub call_id : u64 , pub function_name : String , pub parent_id : Option < u64 > , pub depth : usize , pub duration_ms : u128 , pub success : bool , }
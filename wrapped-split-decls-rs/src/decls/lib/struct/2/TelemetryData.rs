use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone)] pub struct TelemetryData { pub function_name : String , pub start_time : SystemTime , pub duration_ms : u128 , pub input_hash : String , pub output_hash : String , pub success : bool , pub call_id : u64 , }
}
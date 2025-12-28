use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Default)] pub struct ExecutionState { pub current_function : Option < String > , pub call_stack : Vec < String > , pub data_captured : HashMap < String , String > , }
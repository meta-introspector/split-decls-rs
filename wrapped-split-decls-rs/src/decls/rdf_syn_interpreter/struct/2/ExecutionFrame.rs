use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone)] pub struct ExecutionFrame { function_name : String , variables : HashMap < String , SynValue > , current_statement : usize , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub struct SyscallWrapper { pub original_call : String , pub wrapper_macro : String , pub oracle_type : OracleType , pub safety_wrapper : String , pub mock_implementation : Option < String > , pub dao_policy : Option < String > , }
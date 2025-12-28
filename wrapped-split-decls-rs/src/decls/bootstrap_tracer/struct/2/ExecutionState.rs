use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize , Clone)] pub struct ExecutionState { pub id : String , pub step : String , pub inputs : Vec < String > , pub outputs : Vec < String > , pub dependencies : Vec < String > , pub timestamp : u64 , pub success : bool , }
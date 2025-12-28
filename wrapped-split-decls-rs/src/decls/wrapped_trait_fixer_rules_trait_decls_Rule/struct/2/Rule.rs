use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Deserialize)] pub struct Rule { pub kind : RuleKind , pub trait_name : Vec < String > , # [serde (default)] pub apply_to : Vec < String > , pub condition : String , }
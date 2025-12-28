use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize)] pub struct TraitMethod { pub name : String , pub inputs : Vec < String > , pub output : String , pub original_syscall : String , }
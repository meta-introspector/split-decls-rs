use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct CompilerPhase { pub name : String , pub crates : Vec < String > , pub complexity : f64 , }
}
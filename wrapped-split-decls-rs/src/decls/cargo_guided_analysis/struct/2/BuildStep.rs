use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub struct BuildStep { pub step_number : usize , pub action : String , pub target : String , pub duration_estimate : Option < f64 > , }
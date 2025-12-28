use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Deserialize , Serialize , Clone)] pub struct GoalConfig { # [serde (rename = "original-goal")] pub original_goal : Option < String > , pub workflow : Workflow , }
}
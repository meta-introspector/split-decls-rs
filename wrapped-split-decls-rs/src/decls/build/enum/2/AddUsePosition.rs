use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Deserialize)] pub enum AddUsePosition { # [serde (rename = "start")] Start , # [serde (rename = "end")] End , # [serde (rename = "after_use_path")] AfterUsePath (String) , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Deserialize)] pub enum AddUsePosition { # [serde (rename = "start")] Start , # [serde (rename = "end")] End , # [serde (rename = "after_use_path")] AfterUsePath (String) , }
}
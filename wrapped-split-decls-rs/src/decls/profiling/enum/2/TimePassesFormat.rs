use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Which format to use for `-Z time-passes`"] # [derive (Clone , Copy , PartialEq , Hash , Debug)] pub enum TimePassesFormat { # [doc = " Emit human readable text"] Text , # [doc = " Emit structured JSON"] Json , }
}
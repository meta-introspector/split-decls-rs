use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Enum for the debug hex flags."] # [derive (Copy , Clone , Debug , PartialEq)] pub enum DebugHex { # [doc = " The `x` flag in `{:x?}`."] Lower , # [doc = " The `X` flag in `{:X?}`."] Upper , }
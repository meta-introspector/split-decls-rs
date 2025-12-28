use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " The report structure containing all discovered macros."] # [derive (Debug , Serialize , Deserialize)] struct MacroReport { macros : Vec < MacroInfo > , }
}
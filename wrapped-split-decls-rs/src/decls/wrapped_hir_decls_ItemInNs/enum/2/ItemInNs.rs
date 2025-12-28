use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] pub enum ItemInNs { Types (ModuleDef) , Values (ModuleDef) , Macros (Macro) , }
}
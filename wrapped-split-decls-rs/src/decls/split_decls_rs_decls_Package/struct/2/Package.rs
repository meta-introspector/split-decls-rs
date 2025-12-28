use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , serde :: Serialize , serde :: Deserialize)] pub struct Package { pub name : String , pub version : String , pub edition : String , # [serde (default)] pub workspace : Option < bool > , }
}
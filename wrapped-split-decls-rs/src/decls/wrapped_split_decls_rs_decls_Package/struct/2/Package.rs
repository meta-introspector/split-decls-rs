use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , serde :: Serialize , serde :: Deserialize)] struct Package { name : String , version : String , edition : String , # [serde (default)] workspace : Option < bool > , }
}
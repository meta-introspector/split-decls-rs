use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Struct to store source location information"] # [derive (Debug , Clone , PartialEq , Eq , Hash , Serialize , Deserialize)] pub struct SourceLocation { pub file : String , pub line : usize , pub column : usize , }
}
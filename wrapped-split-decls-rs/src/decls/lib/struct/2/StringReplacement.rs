use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Defines a single string replacement operation."] # [derive (Debug , Default , Serialize , Deserialize , Clone)] pub struct StringReplacement { pub old : String , pub new : String , }
}
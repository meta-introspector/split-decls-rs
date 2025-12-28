use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A simple wrapper round a string, used for errors reported from"] # [doc = " ffi calls."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct Error { message : String , }
}
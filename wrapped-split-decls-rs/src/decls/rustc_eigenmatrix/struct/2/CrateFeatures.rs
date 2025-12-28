use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone)] pub struct CrateFeatures { pub name : String , pub path : String , pub functions : usize , pub structs : usize , pub enums : usize , pub macros : usize , pub traits : usize , pub impls : usize , pub loc : usize , pub dependencies : Vec < String > , }
}
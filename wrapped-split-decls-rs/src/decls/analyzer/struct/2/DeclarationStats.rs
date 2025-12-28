use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Default , Debug)] pub struct DeclarationStats { pub total_files : usize , pub function_files : usize , pub struct_files : usize , pub enum_files : usize , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Declaration address mapping"] # [derive (Debug , Clone)] struct DeclAddress { name : String , address : String , decl_type : String , source_path : String , }
}
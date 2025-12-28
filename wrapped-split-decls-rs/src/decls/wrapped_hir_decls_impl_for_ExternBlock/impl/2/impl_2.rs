use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl ExternBlock { pub fn module (self , db : & dyn HirDatabase) -> Module { Module { id : self . id . module (db) , } } }
}
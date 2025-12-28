use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Label { pub fn module (self , db : & dyn HirDatabase) -> Module { self . parent (db) . module (db) } pub fn parent (self , _db : & dyn HirDatabase) -> DefWithBody { self . parent . into () } pub fn name (self , db : & dyn HirDatabase) -> Name { let body = db . body (self . parent) ; body [self . label_id] . name . clone () } }
}
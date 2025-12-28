use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl SourceMap { pub fn new () -> Self { Self { mappings : HashMap :: new () , } } pub fn add_mapping (& mut self , offset : usize , location : SourceLocation) { self . mappings . insert (offset , location) ; } pub fn get_location (& self , offset : usize) -> Option < & SourceLocation > { self . mappings . get (& offset) } }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Person { pub fn new (name : String , age : u32) -> Self { Person { name , age } } }
}
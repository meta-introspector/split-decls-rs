use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Data { pub fn new (attrs : & Attributes < '_ > , written : bool) -> Self { let mut span = Self { start : Instant :: now () , kvs : Vec :: new () , written , } ; attrs . record (& mut span) ; span } }
}
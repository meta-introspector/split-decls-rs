use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Type { pub fn add_offset (self , add : isize) -> Self { let offset = match self . offset { - 1 => add , x => add + x , } ; Self { size : self . size , kind : self . kind , child : self . child , offset } } }
}
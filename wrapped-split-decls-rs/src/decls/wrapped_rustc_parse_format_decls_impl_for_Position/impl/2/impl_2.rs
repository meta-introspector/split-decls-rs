use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Position < '_ > { pub fn index (& self) -> Option < usize > { match self { ArgumentIs (i , ..) | ArgumentImplicitlyIs (i) => Some (* i) , _ => None , } } }
}
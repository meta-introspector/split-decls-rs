use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl PartialEq for Ident { fn eq (& self , other : & Ident) -> bool { self . inner == other . inner } }
}
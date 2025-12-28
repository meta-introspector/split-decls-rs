use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Comparator { pub fn parse (text : & str) -> Result < Self , Error > { Comparator :: from_str (text) } pub fn matches (& self , version : & Version) -> bool { eval :: matches_comparator (self , version) } }
}
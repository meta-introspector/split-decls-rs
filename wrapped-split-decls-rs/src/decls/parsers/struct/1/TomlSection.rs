use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct TomlSection { pub items : Punctuated < KeyValue , Token ! [,] > , }
}
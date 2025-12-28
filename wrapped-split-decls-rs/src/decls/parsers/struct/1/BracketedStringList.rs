use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct BracketedStringList { pub list : Punctuated < LitStr , Token ! [,] > , }
}
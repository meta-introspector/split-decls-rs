use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct BracketedStringList { pub list : Punctuated < LitStr , Token ! [,] > , }
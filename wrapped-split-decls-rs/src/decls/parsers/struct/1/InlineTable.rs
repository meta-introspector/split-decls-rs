use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct InlineTable { pub items : Punctuated < KeyValue , Token ! [,] > , }
}
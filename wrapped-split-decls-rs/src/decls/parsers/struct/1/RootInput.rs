use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct RootInput { pub items : Punctuated < RootItem , Token ! [,] > , }
}
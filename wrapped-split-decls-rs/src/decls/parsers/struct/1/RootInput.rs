use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct RootInput { pub items : Punctuated < RootItem , Token ! [,] > , }
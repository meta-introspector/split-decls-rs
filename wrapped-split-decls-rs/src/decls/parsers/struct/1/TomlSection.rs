use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct TomlSection { pub items : Punctuated < KeyValue , Token ! [,] > , }
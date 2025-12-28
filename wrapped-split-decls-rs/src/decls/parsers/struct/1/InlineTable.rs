use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct InlineTable { pub items : Punctuated < KeyValue , Token ! [,] > , }
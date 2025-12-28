use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Symbol { pub fn intern (_s : & str) -> Self { Symbol } }
}
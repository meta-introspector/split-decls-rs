use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl TestStruct { pub fn new (value : i32) -> Self { Self { value } } }
}
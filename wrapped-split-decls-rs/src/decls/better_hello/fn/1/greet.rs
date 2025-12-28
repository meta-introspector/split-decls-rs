use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn greet (name : & str) -> String { format ! ("Hello, {}!" , name) }
}
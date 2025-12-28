use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn hello () -> String { "Hello, World!" . to_string () }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn hello_world () -> String { "Hello, World!" . to_string () }
}
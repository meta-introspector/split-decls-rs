use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn hello () -> String { "Hello, Amazing Universe!" . to_string () }
}
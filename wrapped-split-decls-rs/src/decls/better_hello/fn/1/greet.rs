use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn greet (name : & str) -> String { format ! ("Hello, {}!" , name) }
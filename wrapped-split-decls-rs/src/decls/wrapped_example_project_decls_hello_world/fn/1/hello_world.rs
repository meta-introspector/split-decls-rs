use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: hello_world");
pub fn hello_world () -> String { "Hello, World!" . to_string () }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: terminal_size");
# [cfg (not (any (unix , windows)))] pub fn terminal_size () -> Option < (Width , Height) > { None }
}
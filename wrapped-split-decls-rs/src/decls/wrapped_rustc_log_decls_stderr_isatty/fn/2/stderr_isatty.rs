use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: stderr_isatty");
pub fn stderr_isatty () -> bool { io :: stderr () . is_terminal () }
}
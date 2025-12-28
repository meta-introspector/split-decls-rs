use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: stdout_isatty");
pub fn stdout_isatty () -> bool { io :: stdout () . is_terminal () }
}
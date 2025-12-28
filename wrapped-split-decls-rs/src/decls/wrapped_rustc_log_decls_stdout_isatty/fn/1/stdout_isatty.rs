use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn stdout_isatty () -> bool { io :: stdout () . is_terminal () }
}
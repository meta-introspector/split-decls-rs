use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn stderr_isatty () -> bool { io :: stderr () . is_terminal () }
}
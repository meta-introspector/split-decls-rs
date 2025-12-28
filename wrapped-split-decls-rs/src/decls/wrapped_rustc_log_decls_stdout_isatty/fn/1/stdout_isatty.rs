use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn stdout_isatty () -> bool { io :: stdout () . is_terminal () }
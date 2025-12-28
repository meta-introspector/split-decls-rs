use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
# [cfg (target_os = "unknown")] impl IsTerminal for std :: process :: ChildStderr { # [inline] fn is_terminal (& self) -> bool { false } }
}
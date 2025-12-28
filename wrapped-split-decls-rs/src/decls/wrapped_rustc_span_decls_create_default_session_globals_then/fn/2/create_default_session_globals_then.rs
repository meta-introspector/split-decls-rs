use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Default edition, no source map."] pub fn create_default_session_globals_then < R > (f : impl FnOnce () -> R) -> R { create_session_globals_then (edition :: DEFAULT_EDITION , & [] , None , f) }
}
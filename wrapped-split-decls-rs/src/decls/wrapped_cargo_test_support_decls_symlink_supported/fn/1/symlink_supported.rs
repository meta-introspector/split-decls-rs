use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [cfg (not (windows))] pub fn symlink_supported () -> bool { true }
}
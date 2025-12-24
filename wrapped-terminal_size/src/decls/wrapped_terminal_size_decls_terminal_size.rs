use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(any(unix, windows)))]
pub fn terminal_size() -> Option<(Width, Height)> {
    None
}

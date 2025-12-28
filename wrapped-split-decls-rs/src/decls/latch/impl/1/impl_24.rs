use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AsCoreLatch for OnceLatch { # [inline] fn as_core_latch (& self) -> & CoreLatch { & self . core_latch } }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'r > AsCoreLatch for SpinLatch < 'r > { # [inline] fn as_core_latch (& self) -> & CoreLatch { & self . core_latch } }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl AsCoreLatch for CoreLatch { # [inline] fn as_core_latch (& self) -> & CoreLatch { self } }
}
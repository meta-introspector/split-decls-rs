use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub (super) trait AsCoreLatch { fn as_core_latch (& self) -> & CoreLatch ; }
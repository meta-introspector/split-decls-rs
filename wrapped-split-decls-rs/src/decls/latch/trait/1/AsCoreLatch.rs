use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltrait! {
pub (super) trait AsCoreLatch { fn as_core_latch (& self) -> & CoreLatch ; }
}
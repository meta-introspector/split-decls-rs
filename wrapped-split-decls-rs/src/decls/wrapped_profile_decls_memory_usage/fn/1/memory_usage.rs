use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn memory_usage () -> MemoryUsage { MemoryUsage :: now () }
}
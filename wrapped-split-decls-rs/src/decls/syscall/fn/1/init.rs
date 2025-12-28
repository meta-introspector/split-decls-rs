use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn init () -> SyscallTracker { SyscallTracker :: new () }
}
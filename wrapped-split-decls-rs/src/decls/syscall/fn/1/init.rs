use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn init () -> SyscallTracker { SyscallTracker :: new () }
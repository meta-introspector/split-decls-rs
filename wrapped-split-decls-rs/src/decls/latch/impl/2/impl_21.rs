use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Latch for LockLatch { # [inline] unsafe fn set (this : * const Self) { let mut guard = unsafe { (* this) . m . lock () . unwrap () } ; * guard = true ; unsafe { (* this) . v . notify_all () } ; } }
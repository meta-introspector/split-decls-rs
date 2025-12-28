use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A Latch starts as false and eventually becomes true. You can block"] # [doc = " until it becomes true."] # [derive (Debug)] pub (super) struct LockLatch { m : Mutex < bool > , v : Condvar , }
}
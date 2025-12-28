use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Spin latches are the simplest, most efficient kind, but they do"] # [doc = " not support a `wait()` operation. They just have a boolean flag"] # [doc = " that becomes true when `set()` is called."] # [derive (Debug)] pub (super) struct CoreLatch { state : AtomicUsize , }
}
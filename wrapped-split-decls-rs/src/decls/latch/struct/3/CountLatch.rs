use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Counting latches are used to implement scopes. They track a"] # [doc = " counter. Unlike other latches, calling `set()` does not"] # [doc = " necessarily make the latch be considered `set()`; instead, it just"] # [doc = " decrements the counter. The latch is only \"set\" (in the sense that"] # [doc = " `probe()` returns true) once the counter reaches zero."] # [derive (Debug)] pub (super) struct CountLatch { counter : AtomicUsize , kind : CountLatchKind , }
}
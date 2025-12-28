use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl LockLatch { # [inline] pub (super) fn new () -> LockLatch { LockLatch { m : Mutex :: new (false) , v : Condvar :: new () } } # [doc = " Block until latch is set, then resets this lock latch so it can be reused again."] pub (super) fn wait_and_reset (& self) { let mut guard = self . m . lock () . unwrap () ; while ! * guard { guard = self . v . wait (guard) . unwrap () ; } * guard = false ; } # [doc = " Block until latch is set."] pub (super) fn wait (& self) { let mut guard = self . m . lock () . unwrap () ; while ! * guard { guard = self . v . wait (guard) . unwrap () ; } } }
}
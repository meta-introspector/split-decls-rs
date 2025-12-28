use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Suspends the execution of the current thread until the time-out interval elapses."] pub fn sleep (milliseconds : u32) { unsafe { Sleep (milliseconds) ; } }
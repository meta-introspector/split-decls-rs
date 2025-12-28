use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate () { let thread_pool = ThreadPoolBuilder :: new () . build () . unwrap () ; thread_pool . install (| | { panic ! ("Hello, world!") ; }) ; }
}
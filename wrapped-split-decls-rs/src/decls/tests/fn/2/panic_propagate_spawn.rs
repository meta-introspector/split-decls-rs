use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_spawn () { scope (| s | s . spawn (| _ | panic ! ("Hello, world!"))) ; }
}
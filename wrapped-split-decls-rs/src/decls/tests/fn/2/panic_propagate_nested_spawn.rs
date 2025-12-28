use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_nested_spawn () { scope (| s | s . spawn (| s | s . spawn (| s | s . spawn (| _ | panic ! ("Hello, world!"))))) ; }
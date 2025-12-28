use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_both () { join (| | panic ! ("Hello, world!") , | | panic ! ("Goodbye, world!")) ; }
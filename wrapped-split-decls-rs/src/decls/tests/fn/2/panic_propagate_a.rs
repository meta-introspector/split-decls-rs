use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: panic_propagate_a");
# [test] # [should_panic (expected = "Hello, world!")] fn panic_propagate_a () { join (| | panic ! ("Hello, world!") , | | ()) ; }
}
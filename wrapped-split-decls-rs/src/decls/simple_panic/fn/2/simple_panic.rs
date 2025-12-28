use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: simple_panic");
# [test] # [should_panic (expected = "should panic")] fn simple_panic () { join (| | { } , | | panic ! ("should panic")) ; }
}
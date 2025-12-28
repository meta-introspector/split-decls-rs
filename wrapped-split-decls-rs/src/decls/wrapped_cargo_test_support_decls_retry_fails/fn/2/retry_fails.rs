use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: retry_fails");
# [test] # [should_panic (expected = "test did not finish")] fn retry_fails () { retry (2 , | | None :: < () >) ; }
}
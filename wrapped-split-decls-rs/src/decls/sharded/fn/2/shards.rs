use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: shards");
# [inline] pub fn shards () -> usize { if is_dyn_thread_safe () { return SHARDS ; } 1 }
}
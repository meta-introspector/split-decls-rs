use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: assert_dyn_sync");
pub fn assert_dyn_sync < T : ? Sized + PointeeSized + DynSync > () { }
}
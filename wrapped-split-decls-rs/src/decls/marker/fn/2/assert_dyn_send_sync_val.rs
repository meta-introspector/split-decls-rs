use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: assert_dyn_send_sync_val");
pub fn assert_dyn_send_sync_val < T : ? Sized + PointeeSized + DynSync + DynSend > (_t : & T) { }
}
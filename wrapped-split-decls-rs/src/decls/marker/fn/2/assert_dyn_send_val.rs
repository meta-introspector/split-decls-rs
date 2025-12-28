use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: assert_dyn_send_val");
pub fn assert_dyn_send_val < T : ? Sized + PointeeSized + DynSend > (_t : & T) { }
}
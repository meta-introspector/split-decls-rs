use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: assert_dyn_send");
pub fn assert_dyn_send < T : ? Sized + PointeeSized + DynSend > () { }
}
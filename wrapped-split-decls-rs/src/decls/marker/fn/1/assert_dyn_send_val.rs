use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn assert_dyn_send_val < T : ? Sized + PointeeSized + DynSend > (_t : & T) { }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn assert_dyn_send < T : ? Sized + PointeeSized + DynSend > () { }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn assert_dyn_sync < T : ? Sized + PointeeSized + DynSync > () { }
}
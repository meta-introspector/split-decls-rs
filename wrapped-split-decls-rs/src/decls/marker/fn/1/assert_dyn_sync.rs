use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn assert_dyn_sync < T : ? Sized + PointeeSized + DynSync > () { }
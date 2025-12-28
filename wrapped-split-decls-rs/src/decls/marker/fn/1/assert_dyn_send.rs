use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn assert_dyn_send < T : ? Sized + PointeeSized + DynSend > () { }
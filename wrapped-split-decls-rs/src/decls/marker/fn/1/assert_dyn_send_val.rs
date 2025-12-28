use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn assert_dyn_send_val < T : ? Sized + PointeeSized + DynSend > (_t : & T) { }
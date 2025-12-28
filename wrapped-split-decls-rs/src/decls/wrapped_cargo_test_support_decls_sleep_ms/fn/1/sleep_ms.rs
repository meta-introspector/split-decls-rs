use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn sleep_ms (ms : u64) { :: std :: thread :: sleep (Duration :: from_millis (ms)) ; }
}
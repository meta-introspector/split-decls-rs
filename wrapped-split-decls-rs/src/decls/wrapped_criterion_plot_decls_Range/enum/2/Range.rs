use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Axis range"] # [derive (Clone , Copy)] pub enum Range { # [doc = " Autoscale the axis"] Auto , # [doc = " Set the limits of the axis"] Limits (f64 , f64) , }
}
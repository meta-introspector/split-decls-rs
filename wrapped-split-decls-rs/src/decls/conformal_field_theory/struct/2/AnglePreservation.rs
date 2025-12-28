use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct AnglePreservation { pub c1_angle : f64 , pub c2_angle : f64 , pub preserved : bool , pub error : f64 , }
}
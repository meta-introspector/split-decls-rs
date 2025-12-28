use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub struct PrimaryField { pub name : String , pub conformal_weight : (f64 , f64) , pub source_location : SourceArrow , pub target_location : SourceArrow , }
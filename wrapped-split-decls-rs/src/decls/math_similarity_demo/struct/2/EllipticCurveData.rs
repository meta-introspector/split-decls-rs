use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] struct EllipticCurveData { field : String , conductor_norm : u32 , curves_count : u32 , isogeny_classes : u32 , complexity_ratio : f64 , }
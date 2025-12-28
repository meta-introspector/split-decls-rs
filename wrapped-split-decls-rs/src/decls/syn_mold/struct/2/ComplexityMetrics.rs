use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Default , Clone)] pub struct ComplexityMetrics { pub parse_operations : u32 , pub visit_operations : u32 , pub transform_operations : u32 , pub generation_operations : u32 , pub total_complexity : f64 , pub max_depth : u32 , }
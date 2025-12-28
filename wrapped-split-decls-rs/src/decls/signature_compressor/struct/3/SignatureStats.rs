use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Statistics about signature compression"] # [derive (Debug)] pub struct SignatureStats { pub total_signatures : usize , pub total_frequency : u64 , pub most_common_signature : String , pub most_common_frequency : u64 , pub least_common_signature : String , pub least_common_frequency : u64 , pub compression_ratio : f64 , pub prime_range : (u64 , u64) , }
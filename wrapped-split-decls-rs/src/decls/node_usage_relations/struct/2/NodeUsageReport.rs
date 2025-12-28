use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize)] struct NodeUsageReport { integer_value : i32 , total_usage_count : usize , user_types : HashMap < String , usize > , relations : HashMap < String , usize > , ast_contexts : HashMap < String , usize > , crate_distribution : HashMap < String , usize > , }
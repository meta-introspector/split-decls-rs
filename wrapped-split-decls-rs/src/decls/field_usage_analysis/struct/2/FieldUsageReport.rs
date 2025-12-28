use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize)] struct FieldUsageReport { integer_value : i32 , total_usage_count : usize , field_usage : HashMap < String , usize > , field_types : HashMap < String , usize > , usage_contexts : HashMap < String , usize > , }
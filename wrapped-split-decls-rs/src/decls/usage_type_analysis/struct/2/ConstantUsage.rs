use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] struct ConstantUsage { value : i32 , count : usize , usage_types : Vec < String > , contexts : Vec < String > , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize)] struct NGram { tokens : Vec < String > , count : usize , pattern_type : String , }
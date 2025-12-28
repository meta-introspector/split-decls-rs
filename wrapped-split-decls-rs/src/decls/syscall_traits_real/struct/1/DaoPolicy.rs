use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub struct DaoPolicy { pub allowed_commands : Vec < String > , pub approval_threshold : f64 , }
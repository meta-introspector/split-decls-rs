use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Usage pattern detection"] # [derive (Debug , Clone)] pub struct UsagePattern { pub pattern_type : PatternType , pub frequency : u32 , pub locations : Vec < String > , }
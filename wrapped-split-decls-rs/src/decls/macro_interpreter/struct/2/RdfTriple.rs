use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub struct RdfTriple { pub subject : String , pub predicate : String , pub object : String , pub timestamp : u64 , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Serialize , Deserialize , Clone)] pub struct RdfTriple { pub subject : String , pub predicate : String , pub object : String , }
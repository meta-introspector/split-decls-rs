use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Serialize , Deserialize)] struct CachedMatrix { matrix : Vec < Vec < f64 > > , decl_names : Vec < String > , timestamp : u64 , }
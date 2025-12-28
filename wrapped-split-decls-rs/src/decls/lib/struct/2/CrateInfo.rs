use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Copy , Serialize , Deserialize)] pub struct CrateInfo { pub name : & 'static str , pub path : & 'static str , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Default , Serialize , Deserialize)] struct CratePathsOnly { # [serde (default)] crate_path_overrides : HashMap < String , String > , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub struct CFTBoundaryCondition { pub emoji_8d_field : Vec < String > , pub conformal_center : String , pub arrow_mappings : HashMap < String , String > , pub holographic_ratio : f64 , }
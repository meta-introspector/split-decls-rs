use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub struct CargoBuildOrder { pub crate_name : String , pub crate_path : PathBuf , pub build_order : Vec < BuildStep > , pub dependencies_resolved : Vec < String > , pub dry_run_output : String , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub struct CargoLockPreservation { pub original_lock : PathBuf , pub split_decls_toml : PathBuf , pub bootstrap_stage : String , pub output2_stage : String , pub packages : Vec < PackageField > , pub preservation_map : HashMap < String , PreservationTrace > , }
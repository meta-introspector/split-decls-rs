use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct CargoGuidedAnalysis { pub root_path : PathBuf , pub cargo_lock : CargoLockPreservation , pub build_orders : HashMap < String , CargoBuildOrder > , pub source_analysis_queue : Vec < PathBuf > , }
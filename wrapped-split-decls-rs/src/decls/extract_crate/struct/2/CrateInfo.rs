use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] struct CrateInfo { name : String , path : PathBuf , dependencies : HashSet < String > , dev_dependencies : HashSet < String > , }
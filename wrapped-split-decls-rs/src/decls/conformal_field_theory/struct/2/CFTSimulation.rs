use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Serialize , Deserialize)] pub struct CFTSimulation { pub rustc_cft : ConformalFieldTheory , pub output2_cft : ConformalFieldTheory , pub neutral_space : Level8DPoint , pub conformal_map : ConformalMap , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , PartialEq , Eq)] pub enum KeepCfg { No , Yes , IfChanged , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [cfg_attr (feature = "nightly" , derive (HashStable_Generic))] # [derive (Copy , Clone , PartialEq , Eq , Hash , Debug)] pub struct Reg { pub kind : RegKind , pub size : Size , }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Copy , PartialEq , Eq , Hash , Debug)] # [cfg_attr (feature = "nightly" , derive (HashStable_Generic))] pub struct Niche { pub offset : Size , pub value : Primitive , pub valid_range : WrappingRange , }
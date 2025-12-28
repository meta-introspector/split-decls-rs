use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [cfg_attr (feature = "nightly" , derive (HashStable_Generic))] # [derive (Copy , Clone , PartialEq , Eq , Hash , Debug)] pub enum RegKind { Integer , Float , Vector , }
}
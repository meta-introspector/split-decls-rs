use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " A set of cfg-overrides per crate."] # [derive (Default , Debug , Clone , Eq , PartialEq)] pub struct CfgOverrides { # [doc = " A global set of overrides matching all crates."] pub global : cfg :: CfgDiff , # [doc = " A set of overrides matching specific crates."] pub selective : rustc_hash :: FxHashMap < String , cfg :: CfgDiff > , }
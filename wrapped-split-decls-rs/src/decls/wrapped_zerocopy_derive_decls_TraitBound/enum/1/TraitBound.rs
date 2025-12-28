use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Eq , PartialEq)] enum TraitBound { Slf , Other (Trait) , }
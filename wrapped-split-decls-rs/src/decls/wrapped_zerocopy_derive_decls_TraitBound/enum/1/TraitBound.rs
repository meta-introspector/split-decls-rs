use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Eq , PartialEq)] enum TraitBound { Slf , Other (Trait) , }
}
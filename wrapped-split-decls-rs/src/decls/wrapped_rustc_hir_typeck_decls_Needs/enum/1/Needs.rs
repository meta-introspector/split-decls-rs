use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum Needs { MutPlace , None , }
}
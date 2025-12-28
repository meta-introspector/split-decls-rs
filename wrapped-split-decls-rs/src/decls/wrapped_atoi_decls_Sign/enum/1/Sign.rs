use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Representation of a numerical sign"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Sign { Plus , Minus , }
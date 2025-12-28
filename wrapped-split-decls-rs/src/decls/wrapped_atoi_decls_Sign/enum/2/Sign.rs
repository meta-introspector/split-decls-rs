use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Representation of a numerical sign"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Sign { Plus , Minus , }
}
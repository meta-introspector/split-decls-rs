use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Axis scale"] # [allow (missing_docs)] # [derive (Clone , Copy)] pub enum Scale { Linear , Logarithmic , }
}
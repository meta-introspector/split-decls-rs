use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Debug , Default , PartialEq , Eq)] pub enum InvocationStrategy { Once , # [default] PerWorkspace , }
}
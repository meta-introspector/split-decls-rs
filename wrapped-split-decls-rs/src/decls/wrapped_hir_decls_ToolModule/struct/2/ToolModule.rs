use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct ToolModule { krate : base_db :: Crate , idx : u32 , }
}
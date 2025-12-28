use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct ExpandedArm (pub ast :: Arm) ;
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
pub struct InactiveReason { enabled : Vec < CfgAtom > , disabled : Vec < CfgAtom > , }
}
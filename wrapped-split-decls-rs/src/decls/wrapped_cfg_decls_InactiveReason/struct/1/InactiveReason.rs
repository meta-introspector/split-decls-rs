use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct InactiveReason { enabled : Vec < CfgAtom > , disabled : Vec < CfgAtom > , }
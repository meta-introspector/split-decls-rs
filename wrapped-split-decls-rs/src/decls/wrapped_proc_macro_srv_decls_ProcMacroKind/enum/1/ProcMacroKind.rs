use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Eq , PartialEq , Debug)] pub enum ProcMacroKind { CustomDerive , Attr , Bang , }
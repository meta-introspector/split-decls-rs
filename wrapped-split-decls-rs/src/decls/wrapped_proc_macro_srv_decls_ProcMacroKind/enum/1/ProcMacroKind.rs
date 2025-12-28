use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , Eq , PartialEq , Debug)] pub enum ProcMacroKind { CustomDerive , Attr , Bang , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone , PartialEq , Eq)] pub enum ProcMacroServerChoice { Sysroot , Explicit (AbsPathBuf) , None , }
}
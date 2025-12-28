use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " Simulate the DebugFile enum from wrapped addr2line  "] # [derive (Debug , Clone , Copy , PartialEq , Eq)] enum DebugFile { Primary , Supplementary , Dwo , }
}
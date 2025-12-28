use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug , Clone , Copy , PartialEq , Eq)] enum DebugFile { Primary , Supplementary , Dwo , }
}
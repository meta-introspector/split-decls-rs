use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub struct LineRange { pub start : LinePosition , pub end : LinePosition , }
}
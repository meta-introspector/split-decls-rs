use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , PartialEq , Eq)] pub struct Error < O , E > { pub error : E , pub backtrace : Vec < O > , }
}
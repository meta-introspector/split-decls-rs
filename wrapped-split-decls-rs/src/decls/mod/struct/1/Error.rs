use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , PartialEq , Eq)] pub struct Error < O , E > { pub error : E , pub backtrace : Vec < O > , }
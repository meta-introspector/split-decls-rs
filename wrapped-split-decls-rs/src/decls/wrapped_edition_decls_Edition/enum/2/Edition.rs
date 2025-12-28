use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (u8)] pub enum Edition { Edition2015 = 0 , Edition2018 , Edition2021 , Edition2024 , }
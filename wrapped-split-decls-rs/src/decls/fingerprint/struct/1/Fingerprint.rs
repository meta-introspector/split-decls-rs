use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Eq , PartialEq , Ord , PartialOrd , Debug , Clone , Copy)] # [repr (C)] pub struct Fingerprint (u64 , u64) ;
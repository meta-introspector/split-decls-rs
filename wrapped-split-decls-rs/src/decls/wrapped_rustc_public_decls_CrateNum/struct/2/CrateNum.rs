use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " The number that identifies a crate."] # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub struct CrateNum (pub usize , ThreadLocalIndex) ;
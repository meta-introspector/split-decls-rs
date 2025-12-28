use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Debug , PartialEq)] pub enum FnCtxt { Free , Foreign , Assoc (AssocCtxt) , }
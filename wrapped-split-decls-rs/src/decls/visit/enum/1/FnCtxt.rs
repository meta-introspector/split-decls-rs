use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , Debug , PartialEq)] pub enum FnCtxt { Free , Foreign , Assoc (AssocCtxt) , }
}
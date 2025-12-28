use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " The error type returned when a key is found to be weak."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub struct WeakKeyError ;
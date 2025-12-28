use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Used as a return value to signify a fatal error occurred."] # [derive (Copy , Clone , Debug)] # [must_use] pub struct FatalError ;
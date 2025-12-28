use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] pub enum PagerankError { CapacityError (String) , }
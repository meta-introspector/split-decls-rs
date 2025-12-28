use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Debug)] pub enum PagerankError { CapacityError (String) , }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub static USING_INTERNAL_FEATURES : AtomicBool = AtomicBool :: new (false) ;
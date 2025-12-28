use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc (hidden)] pub static INTERNAL_RETRY : AtomicBool = AtomicBool :: new (true) ;
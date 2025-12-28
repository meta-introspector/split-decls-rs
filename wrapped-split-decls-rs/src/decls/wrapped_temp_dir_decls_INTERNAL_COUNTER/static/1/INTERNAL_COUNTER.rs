use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc (hidden)] pub static INTERNAL_COUNTER : AtomicU32 = AtomicU32 :: new (0) ;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Minimum value for max threads config"] # [cfg (not (target_family = "wasm"))] const MIN_MAX_THREADS : usize = 1 ;
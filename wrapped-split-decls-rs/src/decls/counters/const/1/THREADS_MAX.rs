use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Max value for the thread counters."] pub (crate) const THREADS_MAX : usize = (1 << THREADS_BITS) - 1 ;
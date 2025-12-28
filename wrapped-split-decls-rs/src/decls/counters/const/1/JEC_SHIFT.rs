use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Bits to shift to select the JEC"] # [doc = " (use JOBS_BITS)."] const JEC_SHIFT : usize = 2 * THREADS_BITS ;
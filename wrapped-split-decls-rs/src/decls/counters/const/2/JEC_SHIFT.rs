use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " Bits to shift to select the JEC"] # [doc = " (use JOBS_BITS)."] const JEC_SHIFT : usize = 2 * THREADS_BITS ;
}
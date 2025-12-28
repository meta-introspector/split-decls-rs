use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " Number of bits used for the thread counters."] # [cfg (target_pointer_width = "64")] const THREADS_BITS : usize = 16 ;
}
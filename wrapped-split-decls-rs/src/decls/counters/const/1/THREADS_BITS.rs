use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [cfg (target_pointer_width = "32")] const THREADS_BITS : usize = 8 ;
}
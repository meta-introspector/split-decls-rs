use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclconst! {
# [doc = " Maximum value for max threads config"] # [cfg (not (target_family = "wasm"))] const MAX_MAX_THREADS : usize = 10000 ;
}
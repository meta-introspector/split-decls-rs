use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn self_install () { let pool = ThreadPoolBuilder :: new () . num_threads (1) . build () . unwrap () ; assert ! (pool . install (|| pool . install (|| true))) ; }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn default_pool () { ThreadPoolBuilder :: default () . build () . unwrap () ; }
}
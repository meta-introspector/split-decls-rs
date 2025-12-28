use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn spawn_then_join_outside_worker () { let (tx , rx) = channel () ; spawn (move | | tx . send (22) . unwrap ()) ; assert_eq ! (22 , rx . recv () . unwrap ()) ; }
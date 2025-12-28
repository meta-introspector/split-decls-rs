use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn spawn_fifo_order () { let vec = test_spawn_order ! (spawn_fifo) ; let expected : Vec < i32 > = (0 .. 10) . collect () ; assert_eq ! (vec , expected) ; }
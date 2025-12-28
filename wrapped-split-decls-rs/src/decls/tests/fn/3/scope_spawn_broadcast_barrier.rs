use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn scope_spawn_broadcast_barrier () { let barrier = Barrier :: new (8) ; let pool = ThreadPoolBuilder :: new () . num_threads (7) . build () . unwrap () ; pool . in_place_scope (| s | { s . spawn_broadcast (| _ , _ | { barrier . wait () ; }) ; barrier . wait () ; }) ; }
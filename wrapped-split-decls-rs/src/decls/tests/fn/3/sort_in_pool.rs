use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn sort_in_pool () { let rng = seeded_rng () ; let mut data : Vec < u32 > = rng . sample_iter (& StandardUniform) . take (12 * 1024) . collect () ; let pool = ThreadPoolBuilder :: new () . build () . unwrap () ; let mut sorted_data = data . clone () ; sorted_data . sort () ; pool . install (| | quick_sort (& mut data)) ; assert_eq ! (data , sorted_data) ; }
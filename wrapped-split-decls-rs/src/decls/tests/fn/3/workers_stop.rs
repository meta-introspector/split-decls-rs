use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn workers_stop () { let registry ; { let thread_pool = ThreadPoolBuilder :: new () . num_threads (22) . build () . unwrap () ; registry = thread_pool . install (| | { join_a_lot (22) ; Arc :: clone (& thread_pool . registry) }) ; assert_eq ! (registry . num_threads () , 22) ; } registry . wait_until_stopped () ; }
}
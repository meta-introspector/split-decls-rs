use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn sleeper_stop () { use std :: { thread , time } ; let registry ; { let thread_pool = ThreadPoolBuilder :: new () . num_threads (22) . build () . unwrap () ; registry = Arc :: clone (& thread_pool . registry) ; thread :: sleep (time :: Duration :: from_secs (1)) ; } registry . wait_until_stopped () ; }
}
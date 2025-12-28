use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: missing_scoped_tls");
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn missing_scoped_tls () { LOCAL . set (& Local (42) , | | { let pool = ThreadPoolBuilder :: new () . build () . expect ("thread pool created") ; pool . install (| | { assert ! (! LOCAL . is_set ()) ; }) ; }) ; }
}
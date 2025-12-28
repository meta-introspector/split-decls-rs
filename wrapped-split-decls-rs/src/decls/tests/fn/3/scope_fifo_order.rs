use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: scope_fifo_order");
# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn scope_fifo_order () { let vec = test_scope_order ! (scope_fifo => spawn_fifo) ; let expected : Vec < i32 > = (0 .. 10) . collect () ; assert_eq ! (vec , expected) ; }
}
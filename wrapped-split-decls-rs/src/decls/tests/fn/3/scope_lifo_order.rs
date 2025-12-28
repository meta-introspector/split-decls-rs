use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: scope_lifo_order");
# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn scope_lifo_order () { let vec = test_scope_order ! (scope => spawn) ; let expected : Vec < i32 > = (0 .. 10) . rev () . collect () ; assert_eq ! (vec , expected) ; }
}
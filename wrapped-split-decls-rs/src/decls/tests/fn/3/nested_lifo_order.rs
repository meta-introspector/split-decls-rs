use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: nested_lifo_order");
# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn nested_lifo_order () { let vec = test_nested_order ! (scope => spawn , scope => spawn) ; let expected : Vec < i32 > = (0 .. 100) . rev () . collect () ; assert_eq ! (vec , expected) ; }
}
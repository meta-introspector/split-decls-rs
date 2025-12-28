use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn lifo_order () { let vec = test_order ! (scope => spawn) ; let expected : Vec < i32 > = (0 .. 100) . rev () . collect () ; assert_eq ! (vec , expected) ; }
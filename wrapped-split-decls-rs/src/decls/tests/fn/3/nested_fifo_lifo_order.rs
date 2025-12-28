use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn nested_fifo_lifo_order () { let vec = test_nested_order ! (scope_fifo => spawn_fifo , scope => spawn) ; let expected : Vec < i32 > = (0 .. 10) . flat_map (| i | (0 .. 10) . rev () . map (move | j | i * 10 + j)) . collect () ; assert_eq ! (vec , expected) ; }
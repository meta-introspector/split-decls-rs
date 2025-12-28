use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn mixed_fifo_lifo_order () { let vec = test_mixed_order ! (scope_fifo => spawn_fifo , scope => spawn) ; let expected = vec ! [- 3 , 0 , - 2 , 1 , - 1 , 2 , 3] ; assert_eq ! (vec , expected) ; }
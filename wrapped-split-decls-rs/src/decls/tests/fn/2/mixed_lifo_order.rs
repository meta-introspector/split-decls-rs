use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn mixed_lifo_order () { let vec = test_mixed_order ! (scope => spawn , scope => spawn) ; let expected = vec ! [- 3 , 2 , - 2 , 1 , - 1 , 3 , 0] ; assert_eq ! (vec , expected) ; }
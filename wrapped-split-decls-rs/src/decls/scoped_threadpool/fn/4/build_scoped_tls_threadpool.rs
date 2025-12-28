use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn build_scoped_tls_threadpool () { LOCAL . set (& Local (42) , | | { LOCAL . with (| x | { ThreadPoolBuilder :: new () . build_scoped (move | thread | LOCAL . set (x , | | thread . run ()) , | pool | { pool . install (| | { assert ! (LOCAL . is_set ()) ; LOCAL . with (| y | { assert_eq ! (x , y) ; }) ; }) ; LOCAL . set (& Local (- 1) , | | { pool . install (| | { assert ! (LOCAL . is_set ()) ; LOCAL . with (| y | { assert_eq ! (x , y) ; }) ; }) ; }) ; } ,) . expect ("thread pool created") ; }) ; }) ; }
}
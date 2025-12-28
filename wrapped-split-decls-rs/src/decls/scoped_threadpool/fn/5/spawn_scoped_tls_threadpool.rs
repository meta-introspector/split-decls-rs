use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn spawn_scoped_tls_threadpool () { LOCAL . set (& Local (42) , | | { LOCAL . with (| x | { thread :: scope (| scope | { let pool = ThreadPoolBuilder :: new () . spawn_handler (move | thread | { scope . builder () . spawn (move | _ | { LOCAL . set (x , | | thread . run ()) }) . map (| _ | ()) }) . build () . expect ("thread pool created") ; pool . install (| | { assert ! (LOCAL . is_set ()) ; LOCAL . with (| y | { assert_eq ! (x , y) ; }) ; }) ; LOCAL . set (& Local (- 1) , | | { pool . install (| | { assert ! (LOCAL . is_set ()) ; LOCAL . with (| y | { assert_eq ! (x , y) ; }) ; }) ; }) ; }) . expect ("scope threads ok") ; }) ; }) ; }
}
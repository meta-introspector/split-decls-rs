// Generated macro for in_place_scope_no_deadlock (function)
macro_rules! Depcrate_thread_pool_testsin_place_scope_no_deadlock {
() => {
// Module: crate::thread_pool::tests
// Provides: {"in_place_scope_no_deadlock"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn in_place_scope_no_deadlock () { let pool = ThreadPoolBuilder :: new () . num_threads (1) . build () . unwrap () ; let (tx , rx) = channel () ; let rx_ref = & rx ; pool . in_place_scope (move | s | { s . spawn (move | _ | { tx . send (()) . unwrap () ; }) ; rx_ref . recv () . unwrap () ; }) ; }
};
}

// Generated macro for self_install (function)
macro_rules! Depcrate_thread_pool_testsself_install {
() => {
// Module: crate::thread_pool::tests
// Provides: {"self_install"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn self_install () { let pool = ThreadPoolBuilder :: new () . num_threads (1) . build () . unwrap () ; assert ! (pool . install (|| pool . install (|| true))) ; }
};
}

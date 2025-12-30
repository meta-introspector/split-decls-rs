// Generated macro for check_thread_pool_new (function)
macro_rules! Depcrate_thread_pool_testscheck_thread_pool_new {
() => {
// Module: crate::thread_pool::tests
// Provides: {"check_thread_pool_new"}
// Dependencies: {}
# [test] # [allow (deprecated)] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn check_thread_pool_new () { let pool = ThreadPool :: new (crate :: Configuration :: new () . num_threads (22)) . unwrap () ; assert_eq ! (pool . current_num_threads () , 22) ; }
};
}

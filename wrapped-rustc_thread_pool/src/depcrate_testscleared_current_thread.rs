// Generated macro for cleared_current_thread (function)
macro_rules! Depcrate_testscleared_current_thread {
() => {
// Module: crate::tests
// Provides: {"cleared_current_thread"}
// Dependencies: {}
# [doc = " Test that custom spawned threads get their `WorkerThread` cleared once"] # [doc = " the pool is done with them, allowing them to be used with rayon again"] # [doc = " later. e.g. WebAssembly want to have their own pool of available threads."] # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn cleared_current_thread () -> Result < () , ThreadPoolBuildError > { let n_threads = 5 ; let mut handles = vec ! [] ; let pool = ThreadPoolBuilder :: new () . num_threads (n_threads) . spawn_handler (| thread | { let handle = std :: thread :: spawn (move | | { thread . run () ; assert_eq ! (crate :: current_thread_index () , None) ; }) ; handles . push (handle) ; Ok (()) }) . build () ? ; assert_eq ! (handles . len () , n_threads) ; pool . install (| | assert ! (crate :: current_thread_index () . is_some ())) ; drop (pool) ; for handle in handles { handle . join () . unwrap () ; } Ok (()) }
};
}

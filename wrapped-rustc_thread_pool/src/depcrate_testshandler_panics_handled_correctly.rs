// Generated macro for handler_panics_handled_correctly (function)
macro_rules! Depcrate_testshandler_panics_handled_correctly {
() => {
// Module: crate::tests
// Provides: {"handler_panics_handled_correctly"}
// Dependencies: {}
# [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn handler_panics_handled_correctly () { let n_threads = 16 ; let n_called = Arc :: new (AtomicUsize :: new (0)) ; let start_barrier = Arc :: new (Barrier :: new (n_threads + 1)) ; let exit_barrier = Arc :: new (Barrier :: new (n_threads + 1)) ; let start_handler = move | _ | { panic ! ("ensure panic handler is called when starting") ; } ; let exit_handler = move | _ | { panic ! ("ensure panic handler is called when exiting") ; } ; let sb = Arc :: clone (& start_barrier) ; let eb = Arc :: clone (& exit_barrier) ; let nc = Arc :: clone (& n_called) ; let panic_handler = move | _ | { let val = nc . fetch_add (1 , Ordering :: SeqCst) ; if val < n_threads { sb . wait () ; } else { eb . wait () ; } } ; let conf = ThreadPoolBuilder :: new () . num_threads (n_threads) . start_handler (start_handler) . exit_handler (exit_handler) . panic_handler (panic_handler) ; { let _ = conf . build () . unwrap () ; start_barrier . wait () ; } exit_barrier . wait () ; assert_eq ! (n_called . load (Ordering :: SeqCst) , 2 * n_threads) ; }
};
}

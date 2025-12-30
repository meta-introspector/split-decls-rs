// Generated macro for panic_thread_name (function)
macro_rules! Depcrate_thread_pool_testspanic_thread_name {
() => {
// Module: crate::thread_pool::tests
// Provides: {"panic_thread_name"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore)] fn panic_thread_name () { let (start_count , start_handler) = count_handler () ; let (exit_count , exit_handler) = count_handler () ; let builder = ThreadPoolBuilder :: new () . num_threads (10) . start_handler (start_handler) . exit_handler (exit_handler) . thread_name (| i | { if i >= 5 { panic ! () ; } format ! ("panic_thread_name#{}" , i) }) ; let pool = crate :: unwind :: halt_unwinding (| | builder . build ()) ; assert ! (pool . is_err () , "thread-name panic should propagate!") ; assert_eq ! (5 , wait_for_counter (start_count)) ; assert_eq ! (5 , wait_for_counter (exit_count)) ; }
};
}

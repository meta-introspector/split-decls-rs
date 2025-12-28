macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! failed_thread_stack {
    () => {
        deps!();
        # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn failed_thread_stack () { let stack_size = :: std :: isize :: MAX as usize ; let (start_count , start_handler) = count_handler () ; let (exit_count , exit_handler) = count_handler () ; let builder = ThreadPoolBuilder :: new () . num_threads (10) . stack_size (stack_size) . start_handler (start_handler) . exit_handler (exit_handler) ; let pool = builder . build () ; assert ! (pool . is_err () , "thread stack should have failed!") ; let start_count = wait_for_counter (start_count) ; assert ! (start_count <= 1) ; assert_eq ! (start_count , wait_for_counter (exit_count)) ; }
    };
}

failed_thread_stack!();
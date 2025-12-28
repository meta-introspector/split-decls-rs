macro_rules! deps {
    () => {
        ThreadPool!();
        Configuration!();
    };
}

macro_rules! check_thread_pool_new {
    () => {
        deps!();
        # [test] # [allow (deprecated)] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn check_thread_pool_new () { let pool = ThreadPool :: new (crate :: Configuration :: new () . num_threads (22)) . unwrap () ; assert_eq ! (pool . current_num_threads () , 22) ; }
    };
}

check_thread_pool_new!()
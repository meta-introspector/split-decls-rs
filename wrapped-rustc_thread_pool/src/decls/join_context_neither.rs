macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! join_context_neither {
    () => {
        deps!();
        # [test] # [ignore] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn join_context_neither () { let pool = ThreadPoolBuilder :: new () . num_threads (1) . build () . unwrap () ; let (a_migrated , b_migrated) = pool . install (| | join_context (| a | a . migrated () , | b | b . migrated ())) ; assert ! (! a_migrated) ; assert ! (! b_migrated) ; }
    };
}

join_context_neither!();
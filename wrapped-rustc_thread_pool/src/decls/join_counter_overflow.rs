macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! join_counter_overflow {
    () => {
        deps!();
        # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn join_counter_overflow () { const MAX : u32 = 500_000 ; let mut i = 0 ; let mut j = 0 ; let pool = ThreadPoolBuilder :: new () . num_threads (2) . build () . unwrap () ; for _ in 0 .. MAX { pool . join (| | i += 1 , | | j += 1) ; } assert_eq ! (i , MAX) ; assert_eq ! (j , MAX) ; }
    };
}

join_counter_overflow!();
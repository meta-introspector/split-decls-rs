macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! scope_spawn_broadcast {
    () => {
        deps!();
        # [test] fn scope_spawn_broadcast () { let pool = ThreadPoolBuilder :: new () . num_threads (7) . build () . unwrap () ; let sum = AtomicUsize :: new (0) ; let n = pool . scope (| s | { s . spawn_broadcast (| _ , ctx | { sum . fetch_add (ctx . index () , Ordering :: Relaxed) ; }) ; crate :: current_num_threads () }) ; assert_eq ! (sum . into_inner () , n * (n - 1) / 2) ; }
    };
}

scope_spawn_broadcast!()
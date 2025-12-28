macro_rules! deps {
    () => {
        QueryMap!();
        QueryJobId!();
    };
}

macro_rules! break_query_cycles {
    () => {
        deps!();
        # [doc = " Detects query cycles by using depth first search over all active query jobs."] # [doc = " If a query cycle is found it will break the cycle by finding an edge which"] # [doc = " uses a query latch and then resuming that waiter."] # [doc = " There may be multiple cycles involved in a deadlock, so this searches"] # [doc = " all active queries for cycles before finally resuming all the waiters at once."] pub fn break_query_cycles < I : Clone + Debug > (query_map : QueryMap < I > , registry : & rustc_thread_pool :: Registry ,) { let mut wakelist = Vec :: new () ; # [allow (rustc :: potential_query_instability)] let mut jobs : Vec < QueryJobId > = query_map . keys () . cloned () . collect () ; let mut found_cycle = false ; while jobs . len () > 0 { if remove_cycle (& query_map , & mut jobs , & mut wakelist) { found_cycle = true ; } } if ! found_cycle { panic ! ("deadlock detected as we're unable to find a query cycle to break\n\
            current query map:\n{:#?}" , query_map) ; } for _ in 0 .. wakelist . len () { rustc_thread_pool :: mark_unblocked (registry) ; } for waiter in wakelist . into_iter () { waiter . condvar . notify_one () ; } }
    };
}

break_query_cycles!()
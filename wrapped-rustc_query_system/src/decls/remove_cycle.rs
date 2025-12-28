macro_rules! deps {
    () => {
        QueryInfo!();
        QueryMap!();
        QueryJobId!();
        CycleError!();
        QueryWaiter!();
    };
}

macro_rules! remove_cycle {
    () => {
        deps!();
        # [doc = " Looks for query cycles starting from the last query in `jobs`."] # [doc = " If a cycle is found, all queries in the cycle is removed from `jobs` and"] # [doc = " the function return true."] # [doc = " If a cycle was not found, the starting query is removed from `jobs` and"] # [doc = " the function returns false."] fn remove_cycle < I : Clone > (query_map : & QueryMap < I > , jobs : & mut Vec < QueryJobId > , wakelist : & mut Vec < Arc < QueryWaiter < I > > > ,) -> bool { let mut visited = FxHashSet :: default () ; let mut stack = Vec :: new () ; if let Some (waiter) = cycle_check (query_map , jobs . pop () . unwrap () , DUMMY_SP , & mut stack , & mut visited) { let (mut spans , queries) : (Vec < _ > , Vec < _ >) = stack . into_iter () . rev () . unzip () ; spans . rotate_right (1) ; let mut stack : Vec < _ > = iter :: zip (spans , queries) . collect () ; for r in & stack { if let Some (pos) = jobs . iter () . position (| j | j == & r . 1) { jobs . remove (pos) ; } } let entry_points = stack . iter () . filter_map (| & (span , query) | { if query . parent (query_map) . is_none () { Some ((span , query , None)) } else { let mut waiters = Vec :: new () ; visit_waiters (query_map , query , | span , waiter | { let mut visited = FxHashSet :: from_iter (stack . iter () . map (| q | q . 1)) ; if connected_to_root (query_map , waiter , & mut visited) { waiters . push ((span , waiter)) ; } None }) ; if waiters . is_empty () { None } else { let waiter = * pick_query (query_map , & waiters , | s | * s) ; Some ((span , query , Some (waiter))) } } }) . collect :: < Vec < (Span , QueryJobId , Option < (Span , QueryJobId) >) > > () ; let (_ , entry_point , usage) = pick_query (query_map , & entry_points , | e | (e . 0 , e . 1)) ; let entry_point_pos = stack . iter () . position (| (_ , query) | query == entry_point) ; if let Some (pos) = entry_point_pos { stack . rotate_left (pos) ; } let usage = usage . as_ref () . map (| (span , query) | (* span , query . query (query_map))) ; let error = CycleError { usage , cycle : stack . iter () . map (| & (s , ref q) | QueryInfo { span : s , query : q . query (query_map) }) . collect () , } ; let (waitee_query , waiter_idx) = waiter . unwrap () ; let waiter = waitee_query . latch (query_map) . unwrap () . extract_waiter (waiter_idx) ; * waiter . cycle . lock () = Some (error) ; wakelist . push (waiter) ; true } else { false } }
    };
}

remove_cycle!()
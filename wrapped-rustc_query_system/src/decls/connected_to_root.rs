macro_rules! deps {
    () => {
        QueryMap!();
        QueryJobId!();
    };
}

macro_rules! connected_to_root {
    () => {
        deps!();
        # [doc = " Finds out if there's a path to the compiler root (aka. code which isn't in a query)"] # [doc = " from `query` without going through any of the queries in `visited`."] # [doc = " This is achieved with a depth first search."] fn connected_to_root < I > (query_map : & QueryMap < I > , query : QueryJobId , visited : & mut FxHashSet < QueryJobId > ,) -> bool { if ! visited . insert (query) { return false ; } if query . parent (query_map) . is_none () { return true ; } visit_waiters (query_map , query , | _ , successor | { connected_to_root (query_map , successor , visited) . then_some (None) }) . is_some () }
    };
}

connected_to_root!();
// Generated macro for connected_to_root (function)
macro_rules! Depcrate_query_jobconnected_to_root {
() => {
// Module: crate::query::job
// Provides: {"connected_to_root"}
// Dependencies: {}
# [doc = " Finds out if there's a path to the compiler root (aka. code which isn't in a query)"] # [doc = " from `query` without going through any of the queries in `visited`."] # [doc = " This is achieved with a depth first search."] fn connected_to_root < I > (query_map : & QueryMap < I > , query : QueryJobId , visited : & mut FxHashSet < QueryJobId > ,) -> bool { if ! visited . insert (query) { return false ; } if query . parent (query_map) . is_none () { return true ; } visit_waiters (query_map , query , | _ , successor | { connected_to_root (query_map , successor , visited) . then_some (None) }) . is_some () }
};
}

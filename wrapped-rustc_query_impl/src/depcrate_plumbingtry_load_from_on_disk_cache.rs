// Generated macro for try_load_from_on_disk_cache (function)
macro_rules! Depcrate_plumbingtry_load_from_on_disk_cache {
() => {
// Module: crate::plumbing
// Provides: {"try_load_from_on_disk_cache"}
// Dependencies: {}
fn try_load_from_on_disk_cache < 'tcx , Q > (query : Q , tcx : TyCtxt < 'tcx > , dep_node : DepNode) where Q : QueryConfig < QueryCtxt < 'tcx > > , { debug_assert ! (tcx . dep_graph . is_green (& dep_node)) ; let key = Q :: Key :: recover (tcx , & dep_node) . unwrap_or_else (| | { panic ! ("Failed to recover key for {:?} with hash {}" , dep_node , dep_node . hash) }) ; if query . cache_on_disk (tcx , & key) { let _ = query . execute_query (tcx , key) ; } }
};
}

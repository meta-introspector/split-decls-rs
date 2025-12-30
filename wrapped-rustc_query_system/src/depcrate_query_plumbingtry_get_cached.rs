// Generated macro for try_get_cached (function)
macro_rules! Depcrate_query_plumbingtry_get_cached {
() => {
// Module: crate::query::plumbing
// Provides: {"try_get_cached"}
// Dependencies: {}
# [doc = " Checks whether there is already a value for this key in the in-memory"] # [doc = " query cache, returning that value if present."] # [doc = ""] # [doc = " (Also performs some associated bookkeeping, if a value was found.)"] # [inline (always)] pub fn try_get_cached < Tcx , C > (tcx : Tcx , cache : & C , key : & C :: Key) -> Option < C :: Value > where C : QueryCache , Tcx : DepContext , { match cache . lookup (key) { Some ((value , index)) => { tcx . profiler () . query_cache_hit (index . into ()) ; tcx . dep_graph () . read_index (index) ; Some (value) } None => None , } }
};
}

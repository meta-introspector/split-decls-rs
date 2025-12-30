// Generated macro for incremental_verify_ich (function)
macro_rules! Depcrate_query_plumbingincremental_verify_ich {
() => {
// Module: crate::query::plumbing
// Provides: {"incremental_verify_ich"}
// Dependencies: {}
# [inline] # [instrument (skip (tcx , dep_graph_data , result , hash_result , format_value) , level = "debug")] pub (crate) fn incremental_verify_ich < Tcx , V > (tcx : Tcx , dep_graph_data : & DepGraphData < Tcx :: Deps > , result : & V , prev_index : SerializedDepNodeIndex , hash_result : Option < fn (& mut StableHashingContext < '_ > , & V) -> Fingerprint > , format_value : fn (& V) -> String ,) where Tcx : DepContext , { if ! dep_graph_data . is_index_green (prev_index) { incremental_verify_ich_not_green (tcx , prev_index) } let new_hash = hash_result . map_or (Fingerprint :: ZERO , | f | { tcx . with_stable_hashing_context (| mut hcx | f (& mut hcx , result)) }) ; let old_hash = dep_graph_data . prev_fingerprint_of (prev_index) ; if new_hash != old_hash { incremental_verify_ich_failed (tcx , prev_index , & | | format_value (result)) ; } }
};
}

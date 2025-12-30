// Generated macro for incremental_verify_ich_not_green (function)
macro_rules! Depcrate_query_plumbingincremental_verify_ich_not_green {
() => {
// Module: crate::query::plumbing
// Provides: {"incremental_verify_ich_not_green"}
// Dependencies: {}
# [cold] # [inline (never)] fn incremental_verify_ich_not_green < Tcx > (tcx : Tcx , prev_index : SerializedDepNodeIndex) where Tcx : DepContext , { panic ! ("fingerprint for green query instance not loaded from cache: {:?}" , tcx . dep_graph () . data () . unwrap () . prev_node_of (prev_index)) }
};
}

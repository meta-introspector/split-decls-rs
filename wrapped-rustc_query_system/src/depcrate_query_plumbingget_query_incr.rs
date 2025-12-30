// Generated macro for get_query_incr (function)
macro_rules! Depcrate_query_plumbingget_query_incr {
() => {
// Module: crate::query::plumbing
// Provides: {"get_query_incr"}
// Dependencies: {}
# [inline (always)] pub fn get_query_incr < Q , Qcx > (query : Q , qcx : Qcx , span : Span , key : Q :: Key , mode : QueryMode ,) -> Option < Q :: Value > where Q : QueryConfig < Qcx > , Qcx : QueryContext , { debug_assert ! (qcx . dep_context () . dep_graph () . is_fully_enabled ()) ; let dep_node = if let QueryMode :: Ensure { check_cache } = mode { let (must_run , dep_node) = ensure_must_run (query , qcx , & key , check_cache) ; if ! must_run { return None ; } dep_node } else { None } ; let (result , dep_node_index) = ensure_sufficient_stack (| | { try_execute_query :: < _ , _ , true > (query , qcx , span , key , dep_node) }) ; if let Some (dep_node_index) = dep_node_index { qcx . dep_context () . dep_graph () . read_index (dep_node_index) } Some (result) }
};
}

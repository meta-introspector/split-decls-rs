// Generated macro for get_query_non_incr (function)
macro_rules! Depcrate_query_plumbingget_query_non_incr {
() => {
// Module: crate::query::plumbing
// Provides: {"get_query_non_incr"}
// Dependencies: {}
# [inline (always)] pub fn get_query_non_incr < Q , Qcx > (query : Q , qcx : Qcx , span : Span , key : Q :: Key) -> Q :: Value where Q : QueryConfig < Qcx > , Qcx : QueryContext , { debug_assert ! (! qcx . dep_context () . dep_graph () . is_fully_enabled ()) ; ensure_sufficient_stack (| | try_execute_query :: < Q , Qcx , false > (query , qcx , span , key , None) . 0) }
};
}

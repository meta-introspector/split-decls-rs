// Generated macro for force_query (function)
macro_rules! Depcrate_query_plumbingforce_query {
() => {
// Module: crate::query::plumbing
// Provides: {"force_query"}
// Dependencies: {}
pub fn force_query < Q , Qcx > (query : Q , qcx : Qcx , key : Q :: Key , dep_node : DepNode) where Q : QueryConfig < Qcx > , Qcx : QueryContext , { if let Some ((_ , index)) = query . query_cache (qcx) . lookup (& key) { qcx . dep_context () . profiler () . query_cache_hit (index . into ()) ; return ; } debug_assert ! (! query . anon ()) ; ensure_sufficient_stack (| | { try_execute_query :: < _ , _ , true > (query , qcx , DUMMY_SP , key , Some (dep_node)) }) ; }
};
}

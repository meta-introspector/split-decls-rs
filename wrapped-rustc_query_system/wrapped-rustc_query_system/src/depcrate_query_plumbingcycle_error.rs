// Generated macro for cycle_error (function)
macro_rules! Depcrate_query_plumbingcycle_error {
() => {
// Module: crate::query::plumbing
// Provides: {"cycle_error"}
// Dependencies: {}
# [cold] # [inline (never)] fn cycle_error < Q , Qcx > (query : Q , qcx : Qcx , try_execute : QueryJobId , span : Span ,) -> (Q :: Value , Option < DepNodeIndex >) where Q : QueryConfig < Qcx > , Qcx : QueryContext , { let query_map = qcx . collect_active_jobs () . ok () . expect ("failed to collect active queries") ; let error = try_execute . find_cycle_in_stack (query_map , & qcx . current_query_job () , span) ; (mk_cycle (query , qcx , error . lift (qcx)) , None) }
};
}

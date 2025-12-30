// Generated macro for mk_cycle (function)
macro_rules! Depcrate_query_plumbingmk_cycle {
() => {
// Module: crate::query::plumbing
// Provides: {"mk_cycle"}
// Dependencies: {}
# [cold] # [inline (never)] fn mk_cycle < Q , Qcx > (query : Q , qcx : Qcx , cycle_error : CycleError) -> Q :: Value where Q : QueryConfig < Qcx > , Qcx : QueryContext , { let error = report_cycle (qcx . dep_context () . sess () , & cycle_error) ; handle_cycle_error (query , qcx , & cycle_error , error) }
};
}

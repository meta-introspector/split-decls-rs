// Generated macro for mir_for_ctfe (function)
macro_rules! Depcratemir_for_ctfe {
() => {
// Module: crate
// Provides: {"mir_for_ctfe"}
// Dependencies: {}
# [doc = " Compute the MIR that is used during CTFE (and thus has no optimizations run on it)"] fn mir_for_ctfe (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> & Body < '_ > { tcx . arena . alloc (inner_mir_for_ctfe (tcx , def_id)) }
};
}

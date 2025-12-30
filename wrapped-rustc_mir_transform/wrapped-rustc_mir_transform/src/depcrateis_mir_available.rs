// Generated macro for is_mir_available (function)
macro_rules! Depcrateis_mir_available {
() => {
// Module: crate
// Provides: {"is_mir_available"}
// Dependencies: {}
fn is_mir_available (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { tcx . mir_keys (()) . contains (& def_id) }
};
}

// Generated macro for nonnull_optimization_guaranteed (function)
macro_rules! Depcrate_typesnonnull_optimization_guaranteed {
() => {
// Module: crate::types
// Provides: {"nonnull_optimization_guaranteed"}
// Dependencies: {}
pub (crate) fn nonnull_optimization_guaranteed < 'tcx > (tcx : TyCtxt < 'tcx > , def : ty :: AdtDef < 'tcx > ,) -> bool { tcx . has_attr (def . did () , sym :: rustc_nonnull_optimization_guaranteed) }
};
}

// Generated macro for ReachableContext (struct)
macro_rules! Depcrate_reachableReachableContext {
() => {
// Module: crate::reachable
// Provides: {"ReachableContext"}
// Dependencies: {}
struct ReachableContext < 'tcx > { tcx : TyCtxt < 'tcx > , maybe_typeck_results : Option < & 'tcx ty :: TypeckResults < 'tcx > > , reachable_symbols : LocalDefIdSet , worklist : Vec < LocalDefId > , any_library : bool , }
};
}

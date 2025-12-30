// Generated macro for check_moves (function)
macro_rules! Depcrate_mono_checks_move_checkcheck_moves {
() => {
// Module: crate::mono_checks::move_check
// Provides: {"check_moves"}
// Dependencies: {}
pub (crate) fn check_moves < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , body : & 'tcx mir :: Body < 'tcx > ,) { let mut visitor = MoveCheckVisitor { tcx , instance , body , move_size_spans : vec ! [] } ; for (bb , data) in traversal :: mono_reachable (body , tcx , instance) { visitor . visit_basic_block_data (bb , data) } }
};
}

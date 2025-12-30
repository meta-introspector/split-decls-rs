// Generated macro for MoveCheckVisitor (struct)
macro_rules! Depcrate_mono_checks_move_checkMoveCheckVisitor {
() => {
// Module: crate::mono_checks::move_check
// Provides: {"MoveCheckVisitor"}
// Dependencies: {}
struct MoveCheckVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , body : & 'tcx mir :: Body < 'tcx > , # [doc = " Spans for move size lints already emitted. Helps avoid duplicate lints."] move_size_spans : Vec < Span > , }
};
}

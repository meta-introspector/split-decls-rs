// Generated macro for CaptureCollector (struct)
macro_rules! Depcrate_upvarsCaptureCollector {
() => {
// Module: crate::upvars
// Provides: {"CaptureCollector"}
// Dependencies: {}
struct CaptureCollector < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , locals : & 'a FxHashSet < HirId > , upvars : FxIndexMap < HirId , hir :: Upvar > , }
};
}

// Generated macro for CloneShimBuilder (struct)
macro_rules! Depcrate_shimCloneShimBuilder {
() => {
// Module: crate::shim
// Provides: {"CloneShimBuilder"}
// Dependencies: {}
struct CloneShimBuilder < 'tcx > { tcx : TyCtxt < 'tcx > , def_id : DefId , local_decls : IndexVec < Local , LocalDecl < 'tcx > > , blocks : IndexVec < BasicBlock , BasicBlockData < 'tcx > > , span : Span , sig : ty :: FnSig < 'tcx > , }
};
}

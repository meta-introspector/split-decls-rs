// Generated macro for DropShimElaborator (struct)
macro_rules! Depcrate_shimDropShimElaborator {
() => {
// Module: crate::shim
// Provides: {"DropShimElaborator"}
// Dependencies: {}
pub (super) struct DropShimElaborator < 'a , 'tcx > { pub body : & 'a Body < 'tcx > , pub patch : MirPatch < 'tcx > , pub tcx : TyCtxt < 'tcx > , pub typing_env : ty :: TypingEnv < 'tcx > , pub produce_async_drops : bool , }
};
}

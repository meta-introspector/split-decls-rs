// Generated macro for Lint (struct)
macro_rules! Depcrate_lintLint {
() => {
// Module: crate::lint
// Provides: {"Lint"}
// Dependencies: {}
struct Lint < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , when : String , body : & 'a Body < 'tcx > , is_fn_like : bool , always_live_locals : & 'a DenseBitSet < Local > , maybe_storage_live : ResultsCursor < 'a , 'tcx , MaybeStorageLive < 'a > > , maybe_storage_dead : ResultsCursor < 'a , 'tcx , MaybeStorageDead < 'a > > , places : FxHashSet < PlaceRef < 'tcx > > , }
};
}

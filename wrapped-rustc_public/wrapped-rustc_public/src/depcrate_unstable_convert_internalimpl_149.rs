// Generated macro for impl_149 (impl)
macro_rules! Depcrate_unstable_convert_internalimpl_149 {
() => {
// Module: crate::unstable::convert::internal
// Provides: {"impl_149"}
// Dependencies: {}
impl RustcInternal for ExistentialPredicate { type T < 'tcx > = rustc_ty :: ExistentialPredicate < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { ExistentialPredicate :: Trait (trait_ref) => { rustc_ty :: ExistentialPredicate :: Trait (trait_ref . internal (tables , tcx)) } ExistentialPredicate :: Projection (proj) => { rustc_ty :: ExistentialPredicate :: Projection (proj . internal (tables , tcx)) } ExistentialPredicate :: AutoTrait (trait_def) => { rustc_ty :: ExistentialPredicate :: AutoTrait (trait_def . 0 . internal (tables , tcx)) } } } }
};
}

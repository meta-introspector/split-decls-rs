// Generated macro for impl_235 (impl)
macro_rules! Depcrate_impls_initializedimpl_235 {
() => {
// Module: crate::impls::initialized
// Provides: {"impl_235"}
// Dependencies: {}
impl < 'a , 'tcx > MaybeUninitializedPlaces < 'a , 'tcx > { pub fn new (tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , move_data : & 'a MoveData < 'tcx >) -> Self { MaybeUninitializedPlaces { tcx , body , move_data , mark_inactive_variants_as_uninit : false , include_inactive_in_otherwise : false , skip_unreachable_unwind : DenseBitSet :: new_empty (body . basic_blocks . len ()) , } } # [doc = " Causes inactive enum variants to be marked as \"maybe uninitialized\" after a switch on an"] # [doc = " enum discriminant."] # [doc = ""] # [doc = " This is correct in a vacuum but is not the default because it causes problems in the borrow"] # [doc = " checker, where this information gets propagated along `FakeEdge`s."] pub fn mark_inactive_variants_as_uninit (mut self) -> Self { self . mark_inactive_variants_as_uninit = true ; self } # [doc = " Ensures definitely inactive variants are included in the set of uninitialized places for"] # [doc = " blocks reached through an `otherwise` edge."] pub fn include_inactive_in_otherwise (mut self) -> Self { self . include_inactive_in_otherwise = true ; self } pub fn skipping_unreachable_unwind (mut self , unreachable_unwind : DenseBitSet < mir :: BasicBlock > ,) -> Self { self . skip_unreachable_unwind = unreachable_unwind ; self } }
};
}

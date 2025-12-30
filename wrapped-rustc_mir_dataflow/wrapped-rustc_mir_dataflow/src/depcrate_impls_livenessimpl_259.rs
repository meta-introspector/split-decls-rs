// Generated macro for impl_259 (impl)
macro_rules! Depcrate_impls_livenessimpl_259 {
() => {
// Module: crate::impls::liveness
// Provides: {"impl_259"}
// Dependencies: {}
impl DefUse { fn apply (state : & mut DenseBitSet < Local > , place : Place < '_ > , context : PlaceContext) { match DefUse :: for_place (place , context) { DefUse :: Def => state . kill (place . local) , DefUse :: Use => state . gen_ (place . local) , DefUse :: PartialWrite | DefUse :: NonUse => { } } } pub fn for_place (place : Place < '_ > , context : PlaceContext) -> DefUse { match context { PlaceContext :: NonUse (_) => DefUse :: NonUse , PlaceContext :: MutatingUse (MutatingUseContext :: Call | MutatingUseContext :: Yield | MutatingUseContext :: AsmOutput | MutatingUseContext :: Store | MutatingUseContext :: Deinit ,) => { if place . is_indirect () { DefUse :: Use } else if place . projection . is_empty () { DefUse :: Def } else { DefUse :: PartialWrite } } PlaceContext :: MutatingUse (MutatingUseContext :: SetDiscriminant) => { if place . is_indirect () { DefUse :: Use } else { DefUse :: PartialWrite } } PlaceContext :: MutatingUse (MutatingUseContext :: RawBorrow | MutatingUseContext :: Borrow | MutatingUseContext :: Drop | MutatingUseContext :: Retag ,) | PlaceContext :: NonMutatingUse (NonMutatingUseContext :: RawBorrow | NonMutatingUseContext :: Copy | NonMutatingUseContext :: Inspect | NonMutatingUseContext :: Move | NonMutatingUseContext :: PlaceMention | NonMutatingUseContext :: FakeBorrow | NonMutatingUseContext :: SharedBorrow ,) => DefUse :: Use , PlaceContext :: MutatingUse (MutatingUseContext :: Projection) | PlaceContext :: NonMutatingUse (NonMutatingUseContext :: Projection) => { unreachable ! ("A projection could be a def or a use and must be handled separately") } } } }
};
}

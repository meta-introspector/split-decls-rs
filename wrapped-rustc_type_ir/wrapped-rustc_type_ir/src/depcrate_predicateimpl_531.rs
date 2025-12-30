// Generated macro for impl_531 (impl)
macro_rules! Depcrate_predicateimpl_531 {
() => {
// Module: crate::predicate
// Provides: {"impl_531"}
// Dependencies: {}
impl < I : Interner > UpcastFrom < I , ty :: Binder < I , TraitRef < I > > > for ty :: Binder < I , TraitPredicate < I > > { fn upcast_from (from : ty :: Binder < I , TraitRef < I > > , _tcx : I) -> Self { from . map_bound (| trait_ref | TraitPredicate { trait_ref , polarity : PredicatePolarity :: Positive , }) } }
};
}

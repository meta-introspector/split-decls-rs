// Generated macro for impl_530 (impl)
macro_rules! Depcrate_predicateimpl_530 {
() => {
// Module: crate::predicate
// Provides: {"impl_530"}
// Dependencies: {}
impl < I : Interner > UpcastFrom < I , TraitRef < I > > for TraitPredicate < I > { fn upcast_from (from : TraitRef < I > , _tcx : I) -> Self { TraitPredicate { trait_ref : from , polarity : PredicatePolarity :: Positive } } }
};
}

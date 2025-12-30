// Generated macro for impl_532 (impl)
macro_rules! Depcrate_predicateimpl_532 {
() => {
// Module: crate::predicate
// Provides: {"impl_532"}
// Dependencies: {}
impl < I : Interner > fmt :: Debug for TraitPredicate < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "TraitPredicate({:?}, polarity:{:?})" , self . trait_ref , self . polarity) } }
};
}

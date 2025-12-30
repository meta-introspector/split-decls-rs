// Generated macro for impl_564 (impl)
macro_rules! Depcrate_predicateimpl_564 {
() => {
// Module: crate::predicate
// Provides: {"impl_564"}
// Dependencies: {}
impl < I : Interner > fmt :: Debug for ProjectionPredicate < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ProjectionPredicate({:?}, {:?})" , self . projection_term , self . term) } }
};
}

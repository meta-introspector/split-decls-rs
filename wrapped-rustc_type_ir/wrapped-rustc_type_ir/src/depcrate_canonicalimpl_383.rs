// Generated macro for impl_383 (impl)
macro_rules! Depcrate_canonicalimpl_383 {
() => {
// Module: crate::canonical
// Provides: {"impl_383"}
// Dependencies: {}
impl < I : Interner , V : fmt :: Display > fmt :: Display for Canonical < I , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Self { value , max_universe , variables } = self ; write ! (f , "Canonical {{ value: {value}, max_universe: {max_universe:?}, variables: {variables:?} }}" ,) } }
};
}

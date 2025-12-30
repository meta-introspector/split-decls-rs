// Generated macro for impl_169 (impl)
macro_rules! Depcrate_usefulnessimpl_169 {
() => {
// Module: crate::usefulness
// Provides: {"impl_169"}
// Dependencies: {}
impl fmt :: Display for PlaceValidity { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let s = match self { ValidOnly => "✓" , MaybeInvalid => "?" , } ; write ! (f , "{s}") } }
};
}

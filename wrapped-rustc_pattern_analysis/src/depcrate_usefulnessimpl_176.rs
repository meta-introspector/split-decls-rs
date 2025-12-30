// Generated macro for impl_176 (impl)
macro_rules! Depcrate_usefulnessimpl_176 {
() => {
// Module: crate::usefulness
// Provides: {"impl_176"}
// Dependencies: {}
impl < 'p , Cx : PatCx > fmt :: Debug for PatStack < 'p , Cx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "+") ? ; for pat in self . iter () { write ! (f , " {pat:?} +") ? ; } Ok (()) } }
};
}

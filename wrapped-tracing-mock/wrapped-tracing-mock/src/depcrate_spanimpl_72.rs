// Generated macro for impl_72 (impl)
macro_rules! Depcrate_spanimpl_72 {
() => {
// Module: crate::span
// Provides: {"impl_72"}
// Dependencies: {}
impl fmt :: Display for NewSpan { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "a new span{}" , self . span . metadata) ? ; if ! self . fields . is_empty () { write ! (f , " with {}" , self . fields) ? ; } Ok (()) } }
};
}

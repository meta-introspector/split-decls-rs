// Generated macro for impl_73 (impl)
macro_rules! Depcrate_hygieneimpl_73 {
() => {
// Module: crate::hygiene
// Provides: {"impl_73"}
// Dependencies: {}
impl fmt :: Display for SyntaxContext { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . is_root () { write ! (f , "ROOT{}" , Edition :: from_u32 (SyntaxContext :: MAX_ID - self . into_u32 ()) . number ()) } else { write ! (f , "{}" , self . into_u32 ()) } } }
};
}

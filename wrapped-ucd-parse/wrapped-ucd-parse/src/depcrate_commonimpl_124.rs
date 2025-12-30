// Generated macro for impl_124 (impl)
macro_rules! Depcrate_commonimpl_124 {
() => {
// Module: crate::common
// Provides: {"impl_124"}
// Dependencies: {}
impl fmt :: Display for Codepoints { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Codepoints :: Single (ref x) => x . fmt (f) , Codepoints :: Range (ref x) => x . fmt (f) , } } }
};
}

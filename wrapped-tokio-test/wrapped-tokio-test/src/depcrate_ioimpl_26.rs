// Generated macro for impl_26 (impl)
macro_rules! Depcrate_ioimpl_26 {
() => {
// Module: crate::io
// Provides: {"impl_26"}
// Dependencies: {}
impl fmt :: Debug for Inner { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . name . is_empty () { write ! (f , "Inner {{...}}") } else { write ! (f , "Inner {{name={}, ...}}" , self . name) } } }
};
}

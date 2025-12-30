// Generated macro for impl_69 (impl)
macro_rules! Depcrate_spanimpl_69 {
() => {
// Module: crate::span
// Provides: {"impl_69"}
// Dependencies: {}
impl fmt :: Display for ExpectedSpan { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . metadata . name . is_some () { write ! (f , "a span{}" , self . metadata) } else { write ! (f , "any span{}" , self . metadata) } } }
};
}

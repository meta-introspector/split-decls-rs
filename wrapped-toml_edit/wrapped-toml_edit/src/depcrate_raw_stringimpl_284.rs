// Generated macro for impl_284 (impl)
macro_rules! Depcrate_raw_stringimpl_284 {
() => {
// Module: crate::raw_string
// Provides: {"impl_284"}
// Dependencies: {}
impl std :: fmt :: Debug for RawString { # [inline] fn fmt (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { match & self . 0 { RawStringInner :: Empty => write ! (formatter , "empty") , RawStringInner :: Explicit (s) => write ! (formatter , "{s:?}") , RawStringInner :: Spanned (s) => write ! (formatter , "{s:?}") , } } }
};
}

// Generated macro for impl_48 (impl)
macro_rules! Depcrate_sourceimpl_48 {
() => {
// Module: crate::source
// Provides: {"impl_48"}
// Dependencies: {}
impl SourceIndex for crate :: lexer :: Token { fn get < 'i > (self , source : & Source < 'i >) -> Option < Raw < 'i > > { (& self) . get (source) } # [cfg (feature = "unsafe")] unsafe fn get_unchecked < 'i > (self , source : & Source < 'i >) -> Raw < 'i > { unsafe { (& self) . get_unchecked (source) } } }
};
}

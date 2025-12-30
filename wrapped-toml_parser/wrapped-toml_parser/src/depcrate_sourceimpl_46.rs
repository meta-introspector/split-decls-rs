// Generated macro for impl_46 (impl)
macro_rules! Depcrate_sourceimpl_46 {
() => {
// Module: crate::source
// Provides: {"impl_46"}
// Dependencies: {}
impl SourceIndex for Span { fn get < 'i > (self , source : & Source < 'i >) -> Option < Raw < 'i > > { (& self) . get (source) } # [cfg (feature = "unsafe")] unsafe fn get_unchecked < 'i > (self , source : & Source < 'i >) -> Raw < 'i > { unsafe { (& self) . get_unchecked (source) } } }
};
}

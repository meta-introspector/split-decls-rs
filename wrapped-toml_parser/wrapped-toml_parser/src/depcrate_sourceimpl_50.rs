// Generated macro for impl_50 (impl)
macro_rules! Depcrate_sourceimpl_50 {
() => {
// Module: crate::source
// Provides: {"impl_50"}
// Dependencies: {}
impl SourceIndex for crate :: parser :: Event { fn get < 'i > (self , source : & Source < 'i >) -> Option < Raw < 'i > > { (& self) . get (source) } # [cfg (feature = "unsafe")] unsafe fn get_unchecked < 'i > (self , source : & Source < 'i >) -> Raw < 'i > { unsafe { (& self) . get_unchecked (source) } } }
};
}

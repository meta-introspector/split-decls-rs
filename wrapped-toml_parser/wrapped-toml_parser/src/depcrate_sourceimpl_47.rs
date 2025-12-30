// Generated macro for impl_47 (impl)
macro_rules! Depcrate_sourceimpl_47 {
() => {
// Module: crate::source
// Provides: {"impl_47"}
// Dependencies: {}
impl SourceIndex for & Span { fn get < 'i > (self , source : & Source < 'i >) -> Option < Raw < 'i > > { let encoding = None ; source . get_raw_str (* self) . map (| s | Raw :: new_unchecked (s , encoding , * self)) } # [cfg (feature = "unsafe")] unsafe fn get_unchecked < 'i > (self , source : & Source < 'i >) -> Raw < 'i > { let encoding = None ; let raw = unsafe { source . get_raw_str_unchecked (* self) } ; Raw :: new_unchecked (raw , encoding , * self) } }
};
}

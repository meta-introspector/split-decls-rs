// Generated macro for impl_98 (impl)
macro_rules! Depcrate_direntimpl_98 {
() => {
// Module: crate::dirent
// Provides: {"impl_98"}
// Dependencies: {}
impl DerefMut for DirentHeader { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self as * mut Self as * mut u8 , size_of :: < Self > ()) } } }
};
}

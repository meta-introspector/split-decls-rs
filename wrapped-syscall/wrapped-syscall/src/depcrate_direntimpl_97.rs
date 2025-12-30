// Generated macro for impl_97 (impl)
macro_rules! Depcrate_direntimpl_97 {
() => {
// Module: crate::dirent
// Provides: {"impl_97"}
// Dependencies: {}
impl Deref for DirentHeader { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const Self as * const u8 , size_of :: < Self > ()) } } }
};
}

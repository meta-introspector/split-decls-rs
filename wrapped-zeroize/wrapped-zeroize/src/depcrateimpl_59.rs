// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
# [cfg (feature = "std")] impl Zeroize for CString { fn zeroize (& mut self) { use core :: mem ; let this = mem :: take (self) ; let mut buf = this . into_bytes_with_nul () ; buf . zeroize () ; let zeroed = CString :: new (buf) . expect ("buf not truncated") ; let _ = mem :: replace (self , zeroed) ; } }
};
}

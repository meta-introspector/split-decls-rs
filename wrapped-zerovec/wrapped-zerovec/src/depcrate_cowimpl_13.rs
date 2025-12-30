// Generated macro for impl_13 (impl)
macro_rules! Depcrate_cowimpl_13 {
() => {
// Module: crate::cow
// Provides: {"impl_13"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Drop for RawVarZeroCow { fn drop (& mut self) { if self . owned { unsafe { let _ = Box :: < [u8] > :: from_raw (self . buf . as_ptr ()) ; } } } }
};
}

// Generated macro for impl_57 (impl)
macro_rules! Depcrate_bytesimpl_57 {
() => {
// Module: crate::bytes
// Provides: {"impl_57"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl From < Box < [u8] > > for Box < Bytes > { fn from (bytes : Box < [u8] >) -> Self { unsafe { Box :: from_raw (Box :: into_raw (bytes) as * mut Bytes) } } }
};
}

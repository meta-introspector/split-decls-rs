// Generated macro for impl_298 (impl)
macro_rules! Depcrate_stream_bytesimpl_298 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_298"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl alloc :: borrow :: ToOwned for Bytes { type Owned = alloc :: vec :: Vec < u8 > ; # [inline] fn to_owned (& self) -> Self :: Owned { alloc :: vec :: Vec :: from (self . as_bytes ()) } }
};
}

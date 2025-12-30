// Generated macro for impl_299 (impl)
macro_rules! Depcrate_stream_bytesimpl_299 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_299"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl core :: borrow :: Borrow < Bytes > for alloc :: vec :: Vec < u8 > { # [inline] fn borrow (& self) -> & Bytes { Bytes :: from_bytes (self . as_slice ()) } }
};
}

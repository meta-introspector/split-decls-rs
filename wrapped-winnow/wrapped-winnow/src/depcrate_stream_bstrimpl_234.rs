// Generated macro for impl_234 (impl)
macro_rules! Depcrate_stream_bstrimpl_234 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_234"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl core :: borrow :: Borrow < BStr > for alloc :: vec :: Vec < u8 > { # [inline] fn borrow (& self) -> & BStr { BStr :: from_bytes (self . as_slice ()) } }
};
}

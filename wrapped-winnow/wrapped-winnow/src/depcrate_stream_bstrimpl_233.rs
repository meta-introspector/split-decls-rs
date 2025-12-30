// Generated macro for impl_233 (impl)
macro_rules! Depcrate_stream_bstrimpl_233 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_233"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl alloc :: borrow :: ToOwned for BStr { type Owned = alloc :: vec :: Vec < u8 > ; # [inline] fn to_owned (& self) -> Self :: Owned { alloc :: vec :: Vec :: from (self . as_bytes ()) } }
};
}

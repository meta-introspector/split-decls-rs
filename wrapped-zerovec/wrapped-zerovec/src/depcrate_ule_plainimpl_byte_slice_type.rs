// Generated macro for impl_byte_slice_type (macro)
macro_rules! Depcrate_ule_plainimpl_byte_slice_type {
() => {
// Module: crate::ule::plain
// Provides: {"impl_byte_slice_type"}
// Dependencies: {}
macro_rules ! impl_byte_slice_type { ($ single_fn : ident , $ type : ty , $ size : literal) => { impl From <$ type > for RawBytesULE <$ size > { # [inline] fn from (value : $ type) -> Self { Self (value . to_le_bytes ()) } } impl AsULE for $ type { type ULE = RawBytesULE <$ size >; # [inline] fn to_unaligned (self) -> Self :: ULE { RawBytesULE (self . to_le_bytes ()) } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { <$ type >:: from_le_bytes (unaligned . 0) } } unsafe impl EqULE for $ type { } impl RawBytesULE <$ size > { pub const fn $ single_fn (v : $ type) -> Self { RawBytesULE (v . to_le_bytes ()) } } } ; }
};
}

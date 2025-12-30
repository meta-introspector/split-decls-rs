// Generated macro for impl_byte_slice_size (macro)
macro_rules! Depcrate_ule_plainimpl_byte_slice_size {
() => {
// Module: crate::ule::plain
// Provides: {"impl_byte_slice_size"}
// Dependencies: {}
macro_rules ! impl_byte_slice_size { ($ unsigned : ty , $ size : literal) => { impl RawBytesULE <$ size > { # [doc = concat ! ("Gets this `RawBytesULE` as a `" , stringify ! ($ unsigned) , "`. This is equivalent to calling [`AsULE::from_unaligned()`] on the appropriately sized type.")] # [inline] pub fn as_unsigned_int (& self) -> $ unsigned { <$ unsigned as $ crate :: ule :: AsULE >:: from_unaligned (* self) } # [doc = concat ! ("Converts a `" , stringify ! ($ unsigned) , "` to a `RawBytesULE`. This is equivalent to calling [`AsULE::to_unaligned()`] on the appropriately sized type.")] # [inline] pub const fn from_aligned (value : $ unsigned) -> Self { Self (value . to_le_bytes ()) } impl_ule_from_array ! ($ unsigned , RawBytesULE <$ size >, RawBytesULE ([0 ; $ size])) ; } } ; }
};
}

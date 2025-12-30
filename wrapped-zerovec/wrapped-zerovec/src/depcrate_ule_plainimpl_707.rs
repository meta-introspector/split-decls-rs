// Generated macro for impl_707 (impl)
macro_rules! Depcrate_ule_plainimpl_707 {
() => {
// Module: crate::ule::plain
// Provides: {"impl_707"}
// Dependencies: {}
impl AsULE for f64 { type ULE = RawBytesULE < 8 > ; # [inline] fn to_unaligned (self) -> Self :: ULE { self . to_bits () . to_unaligned () } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { Self :: from_bits (u64 :: from_unaligned (unaligned)) } }
};
}

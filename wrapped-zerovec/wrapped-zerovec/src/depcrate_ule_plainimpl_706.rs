// Generated macro for impl_706 (impl)
macro_rules! Depcrate_ule_plainimpl_706 {
() => {
// Module: crate::ule::plain
// Provides: {"impl_706"}
// Dependencies: {}
impl AsULE for f32 { type ULE = RawBytesULE < 4 > ; # [inline] fn to_unaligned (self) -> Self :: ULE { self . to_bits () . to_unaligned () } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { Self :: from_bits (u32 :: from_unaligned (unaligned)) } }
};
}

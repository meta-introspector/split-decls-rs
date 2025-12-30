// Generated macro for impl_178 (impl)
macro_rules! Depcrate_zone_zone_name_timestampimpl_178 {
() => {
// Module: crate::zone::zone_name_timestamp
// Provides: {"impl_178"}
// Dependencies: {}
impl AsULE for ZoneNameTimestamp { type ULE = < u32 as AsULE > :: ULE ; # [inline] fn to_unaligned (self) -> Self :: ULE { self . 0 . to_unaligned () } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { Self (u32 :: from_unaligned (unaligned)) } }
};
}

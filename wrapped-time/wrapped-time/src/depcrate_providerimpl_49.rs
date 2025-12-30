// Generated macro for impl_49 (impl)
macro_rules! Depcrate_providerimpl_49 {
() => {
// Module: crate::provider
// Provides: {"impl_49"}
// Dependencies: {}
impl AsULE for Timestamp24 { type ULE = RawBytesULE < 3 > ; # [inline] fn to_unaligned (self) -> Self :: ULE { let RawBytesULE ([a , b , c , _]) = self . 0 . to_unaligned () ; RawBytesULE ([a , b , c]) } # [inline] fn from_unaligned (RawBytesULE ([a , b , c]) : Self :: ULE) -> Self { Self (ZoneNameTimestamp :: from_unaligned (RawBytesULE ([a , b , c , 0]))) } }
};
}

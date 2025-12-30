// Generated macro for impl_108 (impl)
macro_rules! Depcrate_xxhash3impl_108 {
() => {
// Module: crate::xxhash3
// Provides: {"impl_108"}
// Dependencies: {}
impl U8SliceExt for [u8] { # [inline] fn first_u32 (& self) -> Option < u32 > { self . first_chunk () . copied () . map (u32 :: from_le_bytes) } # [inline] fn last_u32 (& self) -> Option < u32 > { self . last_chunk () . copied () . map (u32 :: from_le_bytes) } # [inline] fn first_u64 (& self) -> Option < u64 > { self . first_chunk () . copied () . map (u64 :: from_le_bytes) } # [inline] fn last_u64 (& self) -> Option < u64 > { self . last_chunk () . copied () . map (u64 :: from_le_bytes) } }
};
}

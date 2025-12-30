// Generated macro for impl_114 (impl)
macro_rules! Depcrate_uleimpl_114 {
() => {
// Module: crate::ule
// Provides: {"impl_114"}
// Dependencies: {}
impl < const N : usize > AsULE for TinyAsciiStr < N > { type ULE = Self ; # [inline] fn to_unaligned (self) -> Self :: ULE { self } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { unaligned } }
};
}

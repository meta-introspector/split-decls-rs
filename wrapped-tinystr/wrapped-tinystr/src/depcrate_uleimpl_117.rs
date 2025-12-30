// Generated macro for impl_117 (impl)
macro_rules! Depcrate_uleimpl_117 {
() => {
// Module: crate::ule
// Provides: {"impl_117"}
// Dependencies: {}
impl < const N : usize > AsULE for UnvalidatedTinyAsciiStr < N > { type ULE = Self ; # [inline] fn to_unaligned (self) -> Self :: ULE { self } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { unaligned } }
};
}

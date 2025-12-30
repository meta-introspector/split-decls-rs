// Generated macro for impl_576 (impl)
macro_rules! Depcrate_ule_charsimpl_576 {
() => {
// Module: crate::ule::chars
// Provides: {"impl_576"}
// Dependencies: {}
impl AsULE for char { type ULE = CharULE ; # [inline] fn to_unaligned (self) -> Self :: ULE { CharULE :: from_aligned (self) } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { unaligned . to_char () } }
};
}

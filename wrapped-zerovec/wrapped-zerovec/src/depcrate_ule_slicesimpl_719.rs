// Generated macro for impl_719 (impl)
macro_rules! Depcrate_ule_slicesimpl_719 {
() => {
// Module: crate::ule::slices
// Provides: {"impl_719"}
// Dependencies: {}
impl < T : AsULE , const N : usize > AsULE for [T ; N] { type ULE = [T :: ULE ; N] ; # [inline] fn to_unaligned (self) -> Self :: ULE { self . map (T :: to_unaligned) } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { unaligned . map (T :: from_unaligned) } }
};
}

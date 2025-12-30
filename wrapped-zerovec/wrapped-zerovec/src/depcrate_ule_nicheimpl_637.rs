// Generated macro for impl_637 (impl)
macro_rules! Depcrate_ule_nicheimpl_637 {
() => {
// Module: crate::ule::niche
// Provides: {"impl_637"}
// Dependencies: {}
impl < U : AsULE , const N : usize > AsULE for NichedOption < U , N > where U :: ULE : NicheBytes < N > , { type ULE = NichedOptionULE < U :: ULE , N > ; fn to_unaligned (self) -> Self :: ULE { NichedOptionULE :: new (self . 0 . map (U :: to_unaligned)) } fn from_unaligned (unaligned : Self :: ULE) -> Self { Self (unaligned . get () . map (U :: from_unaligned)) } }
};
}

// Generated macro for MaybeUninitializedPlacesDomain (type)
macro_rules! Depcrate_impls_initializedMaybeUninitializedPlacesDomain {
() => {
// Module: crate::impls::initialized
// Provides: {"MaybeUninitializedPlacesDomain"}
// Dependencies: {}
# [doc = " There can be many more `MovePathIndex` than there are locals in a MIR body."] # [doc = " We use a mixed bitset to avoid paying too high a memory footprint."] pub type MaybeUninitializedPlacesDomain = MixedBitSet < MovePathIndex > ;
};
}

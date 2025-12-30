// Generated macro for impl_779 (impl)
macro_rules! Depcrate_uleimpl_779 {
() => {
// Module: crate::ule
// Provides: {"impl_779"}
// Dependencies: {}
# [cfg (target_endian = "little")] impl < T > SliceAsULE for T where T : EqULE , { # [inline] fn slice_to_unaligned (slice : & [Self]) -> Option < & [Self :: ULE] > { let ule_slice = unsafe { core :: slice :: from_raw_parts (slice . as_ptr () as * const Self :: ULE , slice . len ()) } ; Some (ule_slice) } }
};
}

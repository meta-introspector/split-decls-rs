// Generated macro for bits_for (function)
macro_rules! Depcrate_tagged_ptrbits_for {
() => {
// Module: crate::tagged_ptr
// Provides: {"bits_for"}
// Dependencies: {}
# [doc = " Returns the number of bits available for use for tags in a pointer to `T`"] # [doc = " (this is based on `T`'s alignment)."] pub const fn bits_for < T : ? Sized + Aligned > () -> u32 { crate :: aligned :: align_of :: < T > () . as_nonzero () . trailing_zeros () }
};
}

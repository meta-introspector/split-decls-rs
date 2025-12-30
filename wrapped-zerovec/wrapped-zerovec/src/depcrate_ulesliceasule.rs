// Generated macro for SliceAsULE (trait)
macro_rules! Depcrate_uleSliceAsULE {
() => {
// Module: crate::ule
// Provides: {"SliceAsULE"}
// Dependencies: {}
# [doc = " A trait for a type where aligned slices can be cast to unaligned slices."] # [doc = ""] # [doc = " Auto-implemented on all types implementing [`EqULE`]."] pub trait SliceAsULE where Self : AsULE + Sized , { # [doc = " Converts from `&[Self]` to `&[Self::ULE]` if possible."] # [doc = ""] # [doc = " In general, this function returns `Some` on little-endian and `None` on big-endian."] fn slice_to_unaligned (slice : & [Self]) -> Option < & [Self :: ULE] > ; }
};
}

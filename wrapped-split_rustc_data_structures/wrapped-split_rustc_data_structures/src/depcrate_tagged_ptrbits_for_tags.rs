// Generated macro for bits_for_tags (function)
macro_rules! Depcrate_tagged_ptrbits_for_tags {
() => {
// Module: crate::tagged_ptr
// Provides: {"bits_for_tags"}
// Dependencies: {}
# [doc = " Returns the correct [`Tag::BITS`] constant for a set of tag values."] pub const fn bits_for_tags (mut tags : & [usize]) -> u32 { let mut bits = 0 ; while let & [tag , ref rest @ ..] = tags { tags = rest ; let b = usize :: BITS - tag . leading_zeros () ; if b > bits { bits = b ; } } bits }
};
}

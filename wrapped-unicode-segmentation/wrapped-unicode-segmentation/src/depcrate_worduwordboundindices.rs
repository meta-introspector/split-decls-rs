// Generated macro for UWordBoundIndices (struct)
macro_rules! Depcrate_wordUWordBoundIndices {
() => {
// Module: crate::word
// Provides: {"UWordBoundIndices"}
// Dependencies: {}
# [doc = " External iterator for word boundaries and byte offsets."] # [doc = ""] # [doc = " This struct is created by the [`split_word_bound_indices`] method on the"] # [doc = " [`UnicodeSegmentation`] trait. See its documentation for more."] # [doc = ""] # [doc = " [`split_word_bound_indices`]: trait.UnicodeSegmentation.html#tymethod.split_word_bound_indices"] # [doc = " [`UnicodeSegmentation`]: trait.UnicodeSegmentation.html"] # [derive (Debug , Clone)] pub struct UWordBoundIndices < 'a > { start_offset : usize , iter : UWordBounds < 'a > , }
};
}

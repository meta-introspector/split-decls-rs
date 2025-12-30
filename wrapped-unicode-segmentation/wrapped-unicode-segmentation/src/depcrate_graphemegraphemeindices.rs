// Generated macro for GraphemeIndices (struct)
macro_rules! Depcrate_graphemeGraphemeIndices {
() => {
// Module: crate::grapheme
// Provides: {"GraphemeIndices"}
// Dependencies: {}
# [doc = " External iterator for grapheme clusters and byte offsets."] # [doc = ""] # [doc = " This struct is created by the [`grapheme_indices`] method on the [`UnicodeSegmentation`]"] # [doc = " trait. See its documentation for more."] # [doc = ""] # [doc = " [`grapheme_indices`]: trait.UnicodeSegmentation.html#tymethod.grapheme_indices"] # [doc = " [`UnicodeSegmentation`]: trait.UnicodeSegmentation.html"] # [derive (Debug , Clone)] pub struct GraphemeIndices < 'a > { start_offset : usize , iter : Graphemes < 'a > , }
};
}

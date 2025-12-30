// Generated macro for new_grapheme_indices (function)
macro_rules! Depcrate_graphemenew_grapheme_indices {
() => {
// Module: crate::grapheme
// Provides: {"new_grapheme_indices"}
// Dependencies: {}
# [inline] pub fn new_grapheme_indices (s : & str , is_extended : bool) -> GraphemeIndices < '_ > { GraphemeIndices { start_offset : s . as_ptr () as usize , iter : new_graphemes (s , is_extended) , } }
};
}

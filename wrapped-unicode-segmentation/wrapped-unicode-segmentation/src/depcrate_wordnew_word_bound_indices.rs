// Generated macro for new_word_bound_indices (function)
macro_rules! Depcrate_wordnew_word_bound_indices {
() => {
// Module: crate::word
// Provides: {"new_word_bound_indices"}
// Dependencies: {}
# [inline] pub fn new_word_bound_indices (s : & str) -> UWordBoundIndices < '_ > { UWordBoundIndices { start_offset : s . as_ptr () as usize , iter : new_word_bounds (s) , } }
};
}

// Generated macro for new_sentence_bound_indices (function)
macro_rules! Depcrate_sentencenew_sentence_bound_indices {
() => {
// Module: crate::sentence
// Provides: {"new_sentence_bound_indices"}
// Dependencies: {}
# [inline] pub fn new_sentence_bound_indices (source : & str) -> USentenceBoundIndices < '_ > { USentenceBoundIndices { start_offset : source . as_ptr () as usize , iter : new_sentence_bounds (source) , } }
};
}

// Generated macro for new_unicode_word_indices (function)
macro_rules! Depcrate_wordnew_unicode_word_indices {
() => {
// Module: crate::word
// Provides: {"new_unicode_word_indices"}
// Dependencies: {}
# [inline] pub fn new_unicode_word_indices (s : & str) -> UnicodeWordIndices < '_ > { let inner = if s . is_ascii () { IndicesIter :: Ascii (new_ascii_word_bound_indices (s) . filter (ascii_word_ok)) } else { IndicesIter :: Unicode (new_word_bound_indices (s) . filter (unicode_word_ok)) } ; UnicodeWordIndices { inner } }
};
}

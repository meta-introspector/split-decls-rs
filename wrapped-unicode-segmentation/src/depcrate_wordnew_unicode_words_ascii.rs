// Generated macro for new_unicode_words_ascii (function)
macro_rules! Depcrate_wordnew_unicode_words_ascii {
() => {
// Module: crate::word
// Provides: {"new_unicode_words_ascii"}
// Dependencies: {}
# [inline] fn new_unicode_words_ascii < 'a > (s : & 'a str) -> AsciiWordsIter < 'a > { new_ascii_word_bound_indices (s) . map (strip_pos as fn (_) -> _) . filter (has_ascii_alphanumeric) }
};
}

// Generated macro for new_unicode_words (function)
macro_rules! Depcrate_wordnew_unicode_words {
() => {
// Module: crate::word
// Provides: {"new_unicode_words"}
// Dependencies: {}
# [inline] pub fn new_unicode_words (s : & str) -> UnicodeWords < '_ > { let inner = if s . is_ascii () { WordsIter :: Ascii (new_unicode_words_ascii (s)) } else { WordsIter :: Unicode (new_unicode_words_general (s)) } ; UnicodeWords { inner } }
};
}

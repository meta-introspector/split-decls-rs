// Generated macro for break_words (function)
macro_rules! Depcrate_corebreak_words {
() => {
// Module: crate::core
// Provides: {"break_words"}
// Dependencies: {}
# [doc = " Forcibly break words wider than `line_width` into smaller words."] # [doc = ""] # [doc = " This simply calls [`Word::break_apart`] on words that are too"] # [doc = " wide. This means that no extra `'-'` is inserted, the word is"] # [doc = " simply broken into smaller pieces."] pub fn break_words < 'a , I > (words : I , line_width : usize) -> Vec < Word < 'a > > where I : IntoIterator < Item = Word < 'a > > , { let mut shortened_words = Vec :: new () ; for word in words { if word . width > line_width { shortened_words . extend (word . break_apart (line_width)) ; } else { shortened_words . push (word) ; } } shortened_words }
};
}

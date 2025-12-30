// Generated macro for impl_286 (impl)
macro_rules! Depcrate_wordimpl_286 {
() => {
// Module: crate::word
// Provides: {"impl_286"}
// Dependencies: {}
impl < 'data , 's , Y : RuleBreakType > WordBreakIterator < 'data , 's , Y > { # [doc = " Returns the word type of the segment preceding the current boundary."] # [inline] pub fn word_type (& self) -> WordType { self . 0 . word_type () } # [doc = " Returns an iterator over pairs of boundary position and word type."] pub fn iter_with_word_type (self) -> WordBreakIteratorWithWordType < 'data , 's , Y > { WordBreakIteratorWithWordType (self) } # [doc = " Returns `true` when the segment preceding the current boundary is word-like,"] # [doc = " such as letters, numbers, or CJKV ideographs."] # [inline] pub fn is_word_like (& self) -> bool { self . word_type () . is_word_like () } }
};
}

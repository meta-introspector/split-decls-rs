// Generated macro for WordBreakIterator (struct)
macro_rules! Depcrate_wordWordBreakIterator {
() => {
// Module: crate::word
// Provides: {"WordBreakIterator"}
// Dependencies: {}
# [doc = " Implements the [`Iterator`] trait over the word boundaries of the given string."] # [doc = ""] # [doc = " Lifetimes:"] # [doc = ""] # [doc = " - `'l` = lifetime of the segmenter object from which this iterator was created"] # [doc = " - `'s` = lifetime of the string being segmented"] # [doc = ""] # [doc = " The [`Iterator::Item`] is an [`usize`] representing index of a code unit"] # [doc = " _after_ the boundary (for a boundary at the end of text, this index is the length"] # [doc = " of the [`str`] or array of code units)."] # [doc = ""] # [doc = " For examples of use, see [`WordSegmenter`]."] # [derive (Debug)] pub struct WordBreakIterator < 'data , 's , Y : RuleBreakType > (RuleBreakIterator < 'data , 's , Y >) ;
};
}

// Generated macro for SentenceBreakIterator (struct)
macro_rules! Depcrate_sentenceSentenceBreakIterator {
() => {
// Module: crate::sentence
// Provides: {"SentenceBreakIterator"}
// Dependencies: {}
# [doc = " Implements the [`Iterator`] trait over the sentence boundaries of the given string."] # [doc = ""] # [doc = " Lifetimes:"] # [doc = ""] # [doc = " - `'data` = lifetime of the segmenter object from which this iterator was created"] # [doc = " - `'s` = lifetime of the string being segmented"] # [doc = ""] # [doc = " The [`Iterator::Item`] is an [`usize`] representing index of a code unit"] # [doc = " _after_ the boundary (for a boundary at the end of text, this index is the length"] # [doc = " of the [`str`] or array of code units)."] # [doc = ""] # [doc = " For examples of use, see [`SentenceSegmenter`]."] # [derive (Debug)] pub struct SentenceBreakIterator < 'data , 's , Y : RuleBreakType > (RuleBreakIterator < 'data , 's , Y >) ;
};
}

// Generated macro for GraphemeClusterBreakIterator (struct)
macro_rules! Depcrate_graphemeGraphemeClusterBreakIterator {
() => {
// Module: crate::grapheme
// Provides: {"GraphemeClusterBreakIterator"}
// Dependencies: {}
# [doc = " Implements the [`Iterator`] trait over the grapheme cluster boundaries of the given string."] # [doc = ""] # [doc = " Lifetimes:"] # [doc = ""] # [doc = " - `'data` = lifetime of the segmenter object from which this iterator was created"] # [doc = " - `'s` = lifetime of the string being segmented"] # [doc = ""] # [doc = " The [`Iterator::Item`] is an [`usize`] representing index of a code unit"] # [doc = " _after_ the boundary (for a boundary at the end of text, this index is the length"] # [doc = " of the [`str`] or array of code units)."] # [doc = ""] # [doc = " For examples of use, see [`GraphemeClusterSegmenter`]."] # [derive (Debug)] pub struct GraphemeClusterBreakIterator < 'data , 's , Y : RuleBreakType > (RuleBreakIterator < 'data , 's , Y > ,) ;
};
}

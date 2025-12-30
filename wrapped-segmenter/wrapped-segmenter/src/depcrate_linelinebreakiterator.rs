// Generated macro for LineBreakIterator (struct)
macro_rules! Depcrate_lineLineBreakIterator {
() => {
// Module: crate::line
// Provides: {"LineBreakIterator"}
// Dependencies: {}
# [doc = " Implements the [`Iterator`] trait over the line break opportunities of the given string."] # [doc = ""] # [doc = " Lifetimes:"] # [doc = ""] # [doc = " - `'l` = lifetime of the [`LineSegmenter`] object from which this iterator was created"] # [doc = " - `'s` = lifetime of the string being segmented"] # [doc = ""] # [doc = " The [`Iterator::Item`] is an [`usize`] representing index of a code unit"] # [doc = " _after_ the break (for a break at the end of text, this index is the length"] # [doc = " of the [`str`] or array of code units)."] # [doc = ""] # [doc = " For examples of use, see [`LineSegmenter`]."] # [derive (Debug)] pub struct LineBreakIterator < 'data , 's , Y : LineBreakType > { iter : Y :: IterAttr < 's > , len : usize , current_pos_data : Option < (usize , Y :: CharType) > , result_cache : Vec < usize > , data : & 'data RuleBreakData < 'data > , options : ResolvedLineBreakOptions , complex : ComplexPayloadsBorrowed < 'data > , }
};
}

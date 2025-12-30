// Generated macro for RuleBreakIterator (struct)
macro_rules! Depcrate_rule_segmenterRuleBreakIterator {
() => {
// Module: crate::rule_segmenter
// Provides: {"RuleBreakIterator"}
// Dependencies: {}
# [doc = " Implements the [`Iterator`] trait over the segmenter boundaries of the given string."] # [doc = ""] # [doc = " Lifetimes:"] # [doc = ""] # [doc = " - `'l` = lifetime of the segmenter object from which this iterator was created"] # [doc = " - `'data` = lifetime of data borrowed by segmenter object"] # [doc = "   (this largely exists because segmenter data is invariant due to ZeroMap constraints,"] # [doc = "   think of it as a second 'l)"] # [doc = " - `'s` = lifetime of the string being segmented"] # [doc = ""] # [doc = " The [`Iterator::Item`] is an [`usize`] representing index of a code unit"] # [doc = " _after_ the boundary (for a boundary at the end of text, this index is the length"] # [doc = " of the [`str`] or array of code units)."] # [derive (Debug)] pub struct RuleBreakIterator < 'data , 's , Y : RuleBreakType > { pub (crate) iter : Y :: IterAttr < 's > , pub (crate) len : usize , pub (crate) current_pos_data : Option < (usize , Y :: CharType) > , pub (crate) result_cache : alloc :: vec :: Vec < usize > , pub (crate) data : & 'data RuleBreakData < 'data > , pub (crate) complex : Option < ComplexPayloadsBorrowed < 'data > > , pub (crate) boundary_property : u8 , pub (crate) locale_override : Option < & 'data RuleBreakDataOverride < 'data > > , pub (crate) handle_complex_language : fn (& mut RuleBreakIterator < 'data , 's , Y > , Y :: CharType) -> Option < usize > , }
};
}

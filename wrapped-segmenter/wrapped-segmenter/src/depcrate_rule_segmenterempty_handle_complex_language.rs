// Generated macro for empty_handle_complex_language (function)
macro_rules! Depcrate_rule_segmenterempty_handle_complex_language {
() => {
// Module: crate::rule_segmenter
// Provides: {"empty_handle_complex_language"}
// Dependencies: {}
pub (crate) fn empty_handle_complex_language < Y : RuleBreakType > (_i : & mut RuleBreakIterator < '_ , '_ , Y > , _c : Y :: CharType ,) -> Option < usize > { debug_assert ! (false , "grapheme/sentence segmenters should never need complex language handling") ; None }
};
}

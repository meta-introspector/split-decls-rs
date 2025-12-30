// Generated macro for test_line_ops_inline (function)
macro_rules! Depcrate_text_inlinetest_line_ops_inline {
() => {
// Module: crate::text::inline
// Provides: {"test_line_ops_inline"}
// Dependencies: {}
# [test] fn test_line_ops_inline () { let diff = TextDiff :: from_lines ("Hello World\nsome stuff here\nsome more stuff here\n\nAha stuff here\nand more stuff" , "Stuff\nHello World\nsome amazing stuff here\nsome more stuff here\n" ,) ; assert ! (diff . newline_terminated ()) ; let changes = diff . ops () . iter () . flat_map (| op | diff . iter_inline_changes (op)) . collect :: < Vec < _ > > () ; insta :: assert_debug_snapshot ! (& changes) ; }
};
}

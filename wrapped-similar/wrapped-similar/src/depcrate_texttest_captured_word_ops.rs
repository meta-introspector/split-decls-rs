// Generated macro for test_captured_word_ops (function)
macro_rules! Depcrate_texttest_captured_word_ops {
() => {
// Module: crate::text
// Provides: {"test_captured_word_ops"}
// Dependencies: {}
# [test] fn test_captured_word_ops () { let diff = TextDiff :: from_words ("Hello World\nsome stuff here\nsome more stuff here\n" , "Hello World\nsome amazing stuff here\nsome more stuff here\n" ,) ; let changes = diff . ops () . iter () . flat_map (| op | diff . iter_changes (op)) . collect :: < Vec < _ > > () ; insta :: assert_debug_snapshot ! (& changes) ; }
};
}

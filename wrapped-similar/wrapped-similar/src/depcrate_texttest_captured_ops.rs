// Generated macro for test_captured_ops (function)
macro_rules! Depcrate_texttest_captured_ops {
() => {
// Module: crate::text
// Provides: {"test_captured_ops"}
// Dependencies: {}
# [test] fn test_captured_ops () { let diff = TextDiff :: from_lines ("Hello World\nsome stuff here\nsome more stuff here\n" , "Hello World\nsome amazing stuff here\nsome more stuff here\n" ,) ; insta :: assert_debug_snapshot ! (& diff . ops ()) ; }
};
}

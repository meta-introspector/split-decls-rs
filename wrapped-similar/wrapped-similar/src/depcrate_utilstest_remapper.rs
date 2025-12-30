// Generated macro for test_remapper (function)
macro_rules! Depcrate_utilstest_remapper {
() => {
// Module: crate::utils
// Provides: {"test_remapper"}
// Dependencies: {}
# [test] fn test_remapper () { let a = "foo bar baz" ; let words = a . tokenize_words () ; dbg ! (& words) ; let remap = SliceRemapper :: new (a , & words) ; assert_eq ! (remap . slice (0 .. 3) , Some ("foo bar")) ; assert_eq ! (remap . slice (1 .. 3) , Some (" bar")) ; assert_eq ! (remap . slice (0 .. 1) , Some ("foo")) ; assert_eq ! (remap . slice (0 .. 5) , Some ("foo bar baz")) ; assert_eq ! (remap . slice (0 .. 6) , None) ; }
};
}

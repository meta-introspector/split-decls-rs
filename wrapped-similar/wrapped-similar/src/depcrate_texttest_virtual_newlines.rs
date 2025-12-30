// Generated macro for test_virtual_newlines (function)
macro_rules! Depcrate_texttest_virtual_newlines {
() => {
// Module: crate::text
// Provides: {"test_virtual_newlines"}
// Dependencies: {}
# [test] fn test_virtual_newlines () { let diff = TextDiff :: from_lines ("a\nb" , "a\nc\n") ; assert ! (diff . newline_terminated ()) ; let changes = diff . ops () . iter () . flat_map (| op | diff . iter_changes (op)) . collect :: < Vec < _ > > () ; insta :: assert_debug_snapshot ! (& changes) ; }
};
}

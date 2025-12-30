// Generated macro for test_line_ops (function)
macro_rules! Depcrate_texttest_line_ops {
() => {
// Module: crate::text
// Provides: {"test_line_ops"}
// Dependencies: {}
# [test] fn test_line_ops () { let a = "Hello World\nsome stuff here\nsome more stuff here\n" ; let b = "Hello World\nsome amazing stuff here\nsome more stuff here\n" ; let diff = TextDiff :: from_lines (a , b) ; assert ! (diff . newline_terminated ()) ; let changes = diff . ops () . iter () . flat_map (| op | diff . iter_changes (op)) . collect :: < Vec < _ > > () ; insta :: assert_debug_snapshot ! (& changes) ; # [cfg (feature = "bytes")] { let byte_diff = TextDiff :: from_lines (a . as_bytes () , b . as_bytes ()) ; let byte_changes = byte_diff . ops () . iter () . flat_map (| op | byte_diff . iter_changes (op)) . collect :: < Vec < _ > > () ; for (change , byte_change) in changes . iter () . zip (byte_changes . iter ()) { assert_eq ! (change . to_string_lossy () , byte_change . to_string_lossy ()) ; } } }
};
}

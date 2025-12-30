// Generated macro for test_char_diff (function)
macro_rules! Depcrate_texttest_char_diff {
() => {
// Module: crate::text
// Provides: {"test_char_diff"}
// Dependencies: {}
# [test] fn test_char_diff () { let diff = TextDiff :: from_chars ("Hello World" , "Hallo Welt") ; insta :: assert_debug_snapshot ! (diff . ops ()) ; # [cfg (feature = "bytes")] { let byte_diff = TextDiff :: from_chars ("Hello World" . as_bytes () , "Hallo Welt" . as_bytes ()) ; assert_eq ! (diff . ops () , byte_diff . ops ()) ; } }
};
}

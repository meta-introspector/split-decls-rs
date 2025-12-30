// Generated macro for test_regression_issue_37 (function)
macro_rules! Depcrate_texttest_regression_issue_37 {
() => {
// Module: crate::text
// Provides: {"test_regression_issue_37"}
// Dependencies: {}
# [test] fn test_regression_issue_37 () { let config = TextDiffConfig :: default () ; let diff = config . diff_lines ("\u{18}\n\n" , "\n\n\r") ; let mut output = diff . unified_diff () ; assert_eq ! (output . context_radius (0) . to_string () , "@@ -1 +1,0 @@\n-\u{18}\n@@ -2,0 +2,2 @@\n+\n+\r") ; }
};
}

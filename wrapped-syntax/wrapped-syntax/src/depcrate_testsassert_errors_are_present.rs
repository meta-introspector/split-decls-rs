// Generated macro for assert_errors_are_present (function)
macro_rules! Depcrate_testsassert_errors_are_present {
() => {
// Module: crate::tests
// Provides: {"assert_errors_are_present"}
// Dependencies: {}
fn assert_errors_are_present (errors : & [SyntaxError] , path : & Path) { assert ! (! errors . is_empty () , "There should be errors in the file {:?}" , path . display ()) ; }
};
}

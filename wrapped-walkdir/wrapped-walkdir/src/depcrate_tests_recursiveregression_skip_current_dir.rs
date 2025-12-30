// Generated macro for regression_skip_current_dir (function)
macro_rules! Depcrate_tests_recursiveregression_skip_current_dir {
() => {
// Module: crate::tests::recursive
// Provides: {"regression_skip_current_dir"}
// Dependencies: {}
# [test] fn regression_skip_current_dir () { let dir = Dir :: tmp () ; dir . mkdirp ("foo/a/b") ; dir . mkdirp ("foo/1/2") ; let mut wd = WalkDir :: new (dir . path ()) . max_open (1) . into_iter () ; wd . next () ; wd . next () ; wd . next () ; wd . next () ; wd . skip_current_dir () ; wd . skip_current_dir () ; wd . next () ; }
};
}

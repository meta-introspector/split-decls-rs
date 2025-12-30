// Generated macro for clone (function)
macro_rules! Depcrate_testsclone {
() => {
// Module: crate::tests
// Provides: {"clone"}
// Dependencies: {}
# [expect (clippy :: clone_on_copy , reason = "purpose of the test")] # [test] fn clone () { assert_eq ! (parsing :: component :: Period :: Am . clone () , parsing :: component :: Period :: Am) ; assert ! (crate :: time :: Padding :: Optimize . clone () == crate :: time :: Padding :: Optimize) ; assert ! (matches ! (iso8601 :: ExtendedKind :: Basic . clone () , iso8601 :: ExtendedKind :: Basic)) ; }
};
}

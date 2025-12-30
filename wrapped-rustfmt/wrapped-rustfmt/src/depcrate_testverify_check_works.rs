// Generated macro for verify_check_works (function)
macro_rules! Depcrate_testverify_check_works {
() => {
// Module: crate::test
// Provides: {"verify_check_works"}
// Dependencies: {}
# [test] fn verify_check_works () { init_log () ; let temp_file = make_temp_file ("temp_check.rs") ; Command :: new (rustfmt () . to_str () . unwrap ()) . arg ("--check") . arg (temp_file . path . to_str () . unwrap ()) . status () . expect ("run with check option failed") ; }
};
}

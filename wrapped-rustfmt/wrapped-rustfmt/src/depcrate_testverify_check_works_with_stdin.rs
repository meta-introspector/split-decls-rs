// Generated macro for verify_check_works_with_stdin (function)
macro_rules! Depcrate_testverify_check_works_with_stdin {
() => {
// Module: crate::test
// Provides: {"verify_check_works_with_stdin"}
// Dependencies: {}
# [test] fn verify_check_works_with_stdin () { init_log () ; let mut child = Command :: new (rustfmt () . to_str () . unwrap ()) . arg ("--check") . stdin (Stdio :: piped ()) . stderr (Stdio :: piped ()) . spawn () . expect ("run with check option failed") ; { let stdin = child . stdin . as_mut () . expect ("Failed to open stdin") ; stdin . write_all ("fn main() {}\n" . as_bytes ()) . expect ("Failed to write to rustfmt --check") ; } let output = child . wait_with_output () . expect ("Failed to wait on rustfmt child") ; assert ! (output . status . success ()) ; }
};
}

// Generated macro for stdin_disable_all_formatting_test (function)
macro_rules! Depcrate_teststdin_disable_all_formatting_test {
() => {
// Module: crate::test
// Provides: {"stdin_disable_all_formatting_test"}
// Dependencies: {}
# [test] fn stdin_disable_all_formatting_test () { init_log () ; let input = String :: from ("fn main() { println!(\"This should not be formatted.\"); }") ; let mut child = Command :: new (rustfmt () . to_str () . unwrap ()) . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) . arg ("--config-path=./tests/config/disable_all_formatting.toml") . spawn () . expect ("failed to execute child") ; { let stdin = child . stdin . as_mut () . expect ("failed to get stdin") ; stdin . write_all (input . as_bytes ()) . expect ("failed to write stdin") ; } let output = child . wait_with_output () . expect ("failed to wait on child") ; assert ! (output . status . success ()) ; assert ! (output . stderr . is_empty ()) ; assert_eq ! (input , String :: from_utf8 (output . stdout) . unwrap ()) ; }
};
}

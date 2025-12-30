// Generated macro for stdin_handles_mod_inner_ignore_attr (function)
macro_rules! Depcrate_teststdin_handles_mod_inner_ignore_attr {
() => {
// Module: crate::test
// Provides: {"stdin_handles_mod_inner_ignore_attr"}
// Dependencies: {}
# [test] fn stdin_handles_mod_inner_ignore_attr () { init_log () ; let input = String :: from ("#![rustfmt::skip]\n\nfn    main() {  }") ; let mut child = Command :: new (rustfmt () . to_str () . unwrap ()) . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) . spawn () . expect ("failed to execute child") ; { let stdin = child . stdin . as_mut () . expect ("failed to get stdin") ; stdin . write_all (input . as_bytes ()) . expect ("failed to write stdin") ; } let output = child . wait_with_output () . expect ("failed to wait on child") ; assert ! (output . status . success ()) ; assert ! (output . stderr . is_empty ()) ; assert_eq ! (input , String :: from_utf8 (output . stdout) . unwrap ()) ; }
};
}

// Generated macro for stdin_generated_files_issue_5172 (function)
macro_rules! Depcrate_teststdin_generated_files_issue_5172 {
() => {
// Module: crate::test
// Provides: {"stdin_generated_files_issue_5172"}
// Dependencies: {}
# [test] fn stdin_generated_files_issue_5172 () { init_log () ; let input = Input :: Text ("//@generated\nfn   main() {}" . to_owned ()) ; let mut config = Config :: default () ; config . set () . emit_mode (EmitMode :: Stdout) ; config . set () . format_generated_files (false) ; config . set () . newline_style (NewlineStyle :: Unix) ; let mut buf : Vec < u8 > = vec ! [] ; { let mut session = Session :: new (config , Some (& mut buf)) ; session . format (input) . unwrap () ; assert ! (session . has_no_errors ()) ; } assert_eq ! (String :: from_utf8 (buf) . unwrap () , "<stdin>:\n\n//@generated\nfn main() {}\n" ,) ; }
};
}

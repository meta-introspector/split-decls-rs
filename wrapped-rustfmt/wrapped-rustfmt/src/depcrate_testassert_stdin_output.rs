// Generated macro for assert_stdin_output (function)
macro_rules! Depcrate_testassert_stdin_output {
() => {
// Module: crate::test
// Provides: {"assert_stdin_output"}
// Dependencies: {}
fn assert_stdin_output (source : & Path , expected_filename : & Path , emit_mode : EmitMode , has_diff : bool ,) { let mut config = Config :: default () ; config . set () . newline_style (NewlineStyle :: Unix) ; config . set () . emit_mode (emit_mode) ; let mut source_file = fs :: File :: open (& source) . expect ("couldn't open source") ; let mut source_text = String :: new () ; source_file . read_to_string (& mut source_text) . expect ("Failed reading target") ; let input = Input :: Text (source_text) ; let mut buf : Vec < u8 > = vec ! [] ; { let mut session = Session :: new (config , Some (& mut buf)) ; session . format (input) . unwrap () ; let errors = ReportedErrors { has_diff , .. Default :: default () } ; assert_eq ! (session . errors , errors) ; } let mut expected_file = fs :: File :: open (& expected_filename) . expect ("couldn't open target") ; let mut expected_text = String :: new () ; expected_file . read_to_string (& mut expected_text) . expect ("Failed reading target") ; let output = String :: from_utf8 (buf) . unwrap () ; let compare = make_diff (& expected_text , & output , DIFF_CONTEXT_SIZE) ; if ! compare . is_empty () { let mut failures = HashMap :: new () ; failures . insert (source . to_owned () , compare) ; print_mismatches_default_message (failures) ; panic ! ("Text does not match expected output") ; } }
};
}

// Generated macro for cmd_test_unicode_data (function)
macro_rules! Depcratecmd_test_unicode_data {
() => {
// Module: crate
// Provides: {"cmd_test_unicode_data"}
// Dependencies: {}
fn cmd_test_unicode_data (args : ArgMatches < '_ >) -> Result < () > { let dir = args . ucd_dir () ? ; let mut stdout = io :: stdout () ; for result in UnicodeData :: from_dir (dir) ? { let x : UnicodeData = result ? ; writeln ! (stdout , "{}" , x) ? ; } Ok (()) }
};
}

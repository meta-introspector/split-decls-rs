// Generated macro for print_mismatches (function)
macro_rules! Depcrate_testprint_mismatches {
() => {
// Module: crate::test
// Provides: {"print_mismatches"}
// Dependencies: {}
fn print_mismatches < T : Fn (u32) -> String > (result : HashMap < PathBuf , Vec < Mismatch > > , mismatch_msg_formatter : T ,) { for (_file_name , diff) in result { print_diff (diff , & mismatch_msg_formatter , & Default :: default ()) ; } if let Some (mut t) = term :: stdout () { t . reset () . unwrap_or (()) ; } }
};
}

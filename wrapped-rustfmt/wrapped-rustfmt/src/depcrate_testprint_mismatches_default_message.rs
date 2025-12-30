// Generated macro for print_mismatches_default_message (function)
macro_rules! Depcrate_testprint_mismatches_default_message {
() => {
// Module: crate::test
// Provides: {"print_mismatches_default_message"}
// Dependencies: {}
fn print_mismatches_default_message (result : HashMap < PathBuf , Vec < Mismatch > >) { for (file_name , diff) in result { let mismatch_msg_formatter = | line_num | format ! ("\nMismatch at {}:{}:" , file_name . display () , line_num) ; print_diff (diff , & mismatch_msg_formatter , & Default :: default ()) ; } if let Some (mut t) = term :: stdout () { t . reset () . unwrap_or (()) ; } }
};
}

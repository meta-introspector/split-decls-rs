// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let args = env :: args () . collect :: < Vec < _ > > () ; let str_args = args . iter () . map (| s | s . as_str ()) . collect :: < Vec < _ > > () ; match & str_args . as_slice () [1 ..] { ["eval" , basis , op , inputs @ ..] => do_eval (basis , op , inputs) , _ => { println ! ("{USAGE}\nunrecognized input `{str_args:?}`") ; std :: process :: exit (1) ; } } }
};
}

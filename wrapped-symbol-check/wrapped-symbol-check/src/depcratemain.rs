// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let args = std :: env :: args () . collect :: < Vec < _ > > () ; let args_ref = args . iter () . map (String :: as_str) . collect :: < Vec < _ > > () ; match & args_ref [1 ..] { ["build-and-check" , target , "--" , args @ ..] if ! args . is_empty () => { run_build_and_check (target , args) ; } ["build-and-check" , "--" , args @ ..] if ! args . is_empty () => { let target = & host_target () ; run_build_and_check (target , args) ; } ["check" , paths @ ..] if ! paths . is_empty () => { check_paths (paths) ; } _ => { println ! ("{USAGE}") ; std :: process :: exit (1) ; } } }
};
}

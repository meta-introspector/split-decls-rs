// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
pub fn main () { init_logging () ; let command = match cmdline :: try_parse () { Ok (cmd) => cmd , Err (e) => match e . downcast :: < clap :: error :: Error > () { Ok (clap_err) => clap_err . exit () , Err (some_other_err) => { println ! ("sccache: {some_other_err}") ; for source in some_other_err . chain () . skip (1) { println ! ("sccache: caused by: {source}") ; } std :: process :: exit (1) ; } } , } ; std :: process :: exit (match commands :: run_command (command) { Ok (s) => s , Err (e) => { eprintln ! ("sccache: error: {}" , e) ; for e in e . chain () . skip (1) { eprintln ! ("sccache: caused by: {}" , e) ; } 2 } }) ; }
};
}

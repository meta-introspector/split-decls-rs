// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let exit_code = match main_with_err () { Err (err) => { error ! ("{}" , err) ; 1 } Ok (exit_code) => exit_code , } ; std :: process :: exit (exit_code) ; }
};
}

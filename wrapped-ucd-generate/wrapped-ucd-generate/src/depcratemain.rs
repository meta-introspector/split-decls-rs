// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { if let Err (err) = run () { if err . is_broken_pipe () { process :: exit (0) ; } eprintln ! ("{}" , err) ; process :: exit (1) ; } }
};
}

// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { if let Err (err) = try_main () { eprintln ! ("{}" , err) ; process :: exit (101) ; } }
};
}

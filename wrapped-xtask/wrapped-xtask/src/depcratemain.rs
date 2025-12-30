// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { if let Err (err) = try_main () { eprintln ! ("error: {}" , err) ; std :: process :: exit (1) } }
};
}

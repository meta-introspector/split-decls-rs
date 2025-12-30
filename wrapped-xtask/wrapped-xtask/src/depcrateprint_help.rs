// Generated macro for print_help (function)
macro_rules! Depcrateprint_help {
() => {
// Module: crate
// Provides: {"print_help"}
// Dependencies: {}
fn print_help () { eprintln ! ("Tasks:") ; for task in TASKS { eprintln ! ("  {:20}{}" , task . 0 , task . 2) ; } }
};
}

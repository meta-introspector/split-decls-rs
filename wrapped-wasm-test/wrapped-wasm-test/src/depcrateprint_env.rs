// Generated macro for print_env (function)
macro_rules! Depcrateprint_env {
() => {
// Module: crate
// Provides: {"print_env"}
// Dependencies: {}
pub fn print_env () { println ! ("Arguments:") ; for argument in env :: args () { println ! ("{argument}") ; } println ! ("Environment variables:") ; for (key , value) in env :: vars () { println ! ("{key}: {value}") ; } }
};
}

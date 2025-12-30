// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let sh = Shell :: new () . unwrap () ; let stdout = cmd ! (sh , "echo hello world") . read () . unwrap () ; print ! ("{}\n" , stdout) }
};
}

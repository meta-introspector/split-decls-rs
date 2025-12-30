// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let t = thread :: spawn (| | { let _info : DeriveStruct = serde_json :: from_str (JSON) . unwrap () ; }) ; let _info : DeriveStruct = serde_json :: from_str (JSON) . unwrap () ; t . join () . unwrap () ; }
};
}

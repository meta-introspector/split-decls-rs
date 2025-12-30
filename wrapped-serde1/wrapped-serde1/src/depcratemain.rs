// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let info : DeriveStruct = serde_json :: from_str (JSON) . unwrap () ; println ! ("{}" , info . buffer . len ()) ; }
};
}

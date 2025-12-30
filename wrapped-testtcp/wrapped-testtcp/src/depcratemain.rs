// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let listener = TcpListener :: bind ("0.0.0.0:9975") . unwrap () ; let (mut socket , _) = listener . accept () . unwrap () ; let mut buf = [0u8 ; 1000] ; loop { println ! ("about to read") ; match socket . read (& mut buf) { Err (e) => { println ! ("read err {e:?}") ; break ; } Ok (received) => { print ! ("read {}" , std :: str :: from_utf8 (& buf [.. received]) . unwrap ()) ; if received == 0 { break ; } } } } }
};
}

// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [cfg (feature = "client")] fn main () { let addr = vsock :: VsockAddr :: new (2 , 9975) ; let mut socket = vsock :: VsockStream :: connect (addr) . expect ("connection failed") ; let mut buf = [0u8 ; 1000] ; loop { match socket . read (& mut buf) { Err (e) => { println ! ("read err {e:?}") ; break ; } Ok (received) => { let msg = std :: str :: from_utf8 (& buf [.. received]) . unwrap () ; print ! ("{}" , msg) ; if msg . trim () == "exit" { break ; } } } } }
};
}

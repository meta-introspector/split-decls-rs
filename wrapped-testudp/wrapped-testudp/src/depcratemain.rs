// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let socket = UdpSocket :: bind ("0.0.0.0:9975") . expect ("couldn't bind to address") ; let mut buf = [0 ; 1000] ; loop { match socket . recv_from (& mut buf) { Ok ((received , addr)) => { let msg = std :: str :: from_utf8 (& buf [.. received]) . unwrap () ; match msg . strip_suffix ('\n') { Some (striped_msg) => { println ! ("received \"{striped_msg}\" from {addr}") ; } _ => { println ! ("received \"{msg}\" from {addr}") ; } } socket . send_to (msg . as_bytes () , addr) . expect ("Unable to send message back") ; if msg . starts_with ("exit") { break ; } } Err (e) => { println ! ("recv function failed: {e:?}") ; break ; } } } }
};
}

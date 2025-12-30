// Generated macro for report_handshake_result (function)
macro_rules! Depcratereport_handshake_result {
() => {
// Module: crate
// Provides: {"report_handshake_result"}
// Dependencies: {}
fn report_handshake_result (variant : & str , params : & Parameters , rounds : u64 , timings : Vec < Timings >) { print ! ("{}\t{:?}\t{:?}\t{:?}\tclient\t{}\t{}\t" , variant , params . proto . version , params . proto . key_type , params . proto . ciphersuite , params . client_auth . label () , params . resume . label () ,) ; report_timings ("handshakes/s" , & timings , rounds as f64 , | t | t . client) ; print ! ("{}\t{:?}\t{:?}\t{:?}\tserver\t{}\t{}\t" , variant , params . proto . version , params . proto . key_type , params . proto . ciphersuite , params . client_auth . label () , params . resume . label () ,) ; report_timings ("handshakes/s" , & timings , rounds as f64 , | t | t . server) ; }
};
}

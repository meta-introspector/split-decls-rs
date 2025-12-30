// Generated macro for bench_handshake (function)
macro_rules! Depcratebench_handshake {
() => {
// Module: crate
// Provides: {"bench_handshake"}
// Dependencies: {}
fn bench_handshake (params : & Parameters) { let client_config = params . client_config () ; let server_config = params . server_config () ; let rounds = params . apply_work_multiplier (if params . resume == ResumptionParam :: No { 512 } else { 4096 }) ; bench_handshake_buffered (1 , ResumptionParam :: No , client_config . clone () , server_config . clone () , & params . without_latency_measurement () ,) ; if params . api . use_buffered () { let results = multithreaded (params . threads , & client_config , & server_config , move | client_config , server_config | { bench_handshake_buffered (rounds , params . resume , client_config , server_config , params ,) } ,) ; report_handshake_result ("handshakes" , params , rounds , results) ; } if params . api . use_unbuffered () { let results = multithreaded (params . threads , & client_config , & server_config , move | client_config , server_config | { bench_handshake_unbuffered (rounds , params . resume , client_config , server_config) } ,) ; report_handshake_result ("handshakes-unbuffered" , params , rounds , results) ; } }
};
}

// Generated macro for bench_bulk (function)
macro_rules! Depcratebench_bulk {
() => {
// Module: crate
// Provides: {"bench_bulk"}
// Dependencies: {}
fn bench_bulk (params : & Parameters) { let client_config = params . client_config () ; let server_config = params . server_config () ; let total_data = params . apply_work_multiplier (1024 * 1024 * match params . plaintext_size { ..= 8192 => 64 , _ => 1024 , } ,) ; let rounds = total_data / params . plaintext_size ; if params . api . use_buffered () { let results = multithreaded (params . threads , & client_config , & server_config , move | client_config , server_config | { bench_bulk_buffered (client_config , server_config , params . plaintext_size , rounds) } ,) ; report_bulk_result ("bulk" , params , results , rounds) ; } if params . api . use_unbuffered () { let results = multithreaded (params . threads , & client_config , & server_config , move | client_config , server_config | { bench_bulk_unbuffered (client_config , server_config , params . plaintext_size , rounds) } ,) ; report_bulk_result ("bulk-unbuffered" , params , results , rounds) ; } }
};
}

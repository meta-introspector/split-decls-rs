// Generated macro for all_tests (function)
macro_rules! Depcrateall_tests {
() => {
// Module: crate
// Provides: {"all_tests"}
// Dependencies: {}
fn all_tests (args : & Args) { let provider = args . provider . unwrap_or_else (Provider :: choose_default) ; for bench in ALL_BENCHMARKS . iter () . filter (| t | provider . supports_benchmark (t)) { let params = Parameters :: new (bench , args) . with_plaintext_size (1024 * 1024) ; bench_bulk (& params) ; bench_bulk (& params . with_max_fragment (Some (10000))) ; bench_handshake (& params) ; bench_handshake (& params . with_client_auth (ClientAuth :: Yes)) ; bench_handshake (& params . with_resume (ResumptionParam :: SessionId)) ; bench_handshake (& params . with_client_auth (ClientAuth :: Yes) . with_resume (ResumptionParam :: SessionId) ,) ; bench_handshake (& params . with_resume (ResumptionParam :: Tickets)) ; bench_handshake (& params . with_client_auth (ClientAuth :: Yes) . with_resume (ResumptionParam :: Tickets) ,) ; } }
};
}

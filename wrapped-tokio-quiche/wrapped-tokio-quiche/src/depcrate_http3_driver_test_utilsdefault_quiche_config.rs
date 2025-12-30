// Generated macro for default_quiche_config (function)
macro_rules! Depcrate_http3_driver_test_utilsdefault_quiche_config {
() => {
// Module: crate::http3::driver::test_utils
// Provides: {"default_quiche_config"}
// Dependencies: {}
pub fn default_quiche_config () -> quiche :: Config { let mut config = quiche :: Config :: new (quiche :: PROTOCOL_VERSION) . unwrap () ; config . load_cert_chain_from_pem_file ("examples/cert.crt") . unwrap () ; config . load_priv_key_from_pem_file ("examples/cert.key") . unwrap () ; config . set_application_protos (& [b"h3"]) . unwrap () ; config . set_initial_max_data (1500) ; config . set_initial_max_stream_data_bidi_local (150) ; config . set_initial_max_stream_data_bidi_remote (150) ; config . set_initial_max_stream_data_uni (150) ; config . set_initial_max_streams_bidi (100) ; config . set_initial_max_streams_uni (5) ; config . verify_peer (false) ; config }
};
}

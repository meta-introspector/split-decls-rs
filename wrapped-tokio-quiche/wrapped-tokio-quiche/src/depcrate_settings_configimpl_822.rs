// Generated macro for impl_822 (impl)
macro_rules! Depcrate_settings_configimpl_822 {
() => {
// Module: crate::settings::config
// Provides: {"impl_822"}
// Dependencies: {}
impl Config { pub (crate) fn new (params : & ConnectionParams , socket_capabilities : SocketCapabilities ,) -> QuicResult < Self > { let quic_settings = & params . settings ; let keylog_path = match & quic_settings . keylog_file { Some (f) => Some (Cow :: Borrowed (f . as_ref ())) , None => std :: env :: var_os ("SSLKEYLOGFILE") . map (Cow :: from) , } ; let keylog_file = keylog_path . and_then (| path | if KEYLOGFILE_ENABLED { File :: options () . create (true) . append (true) . open (path) . inspect_err (| e | log :: warn ! ("failed to open SSLKEYLOGFILE" ; "error" => e)) . ok () } else { log :: warn ! ("SSLKEYLOGFILE is set, but `--cfg capture_keylogs` was not enabled. No keys will be logged.") ; None }) ; let SocketCapabilities { has_gso , has_txtime : pacing_offload , has_ippktinfo , has_ipv6pktinfo , .. } = socket_capabilities ; # [cfg (feature = "gcongestion")] let pacing_offload = quic_settings . enable_pacing && pacing_offload ; Ok (Config { quiche_config : make_quiche_config (params , keylog_file . is_some ()) ? , disable_client_ip_validation : quic_settings . disable_client_ip_validation , qlog_dir : quic_settings . qlog_dir . clone () , has_gso , pacing_offload , enable_expensive_packet_count_metrics : quic_settings . enable_expensive_packet_count_metrics , keylog_file , listen_backlog : quic_settings . listen_backlog , handshake_timeout : quic_settings . handshake_timeout , has_ippktinfo , has_ipv6pktinfo , }) } }
};
}

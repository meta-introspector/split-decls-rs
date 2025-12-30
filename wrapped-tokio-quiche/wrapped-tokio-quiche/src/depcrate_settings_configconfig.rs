// Generated macro for Config (struct)
macro_rules! Depcrate_settings_configConfig {
() => {
// Module: crate::settings::config
// Provides: {"Config"}
// Dependencies: {}
# [doc = " Internal representation of the combined configuration for a QUIC connection."] pub (crate) struct Config { pub quiche_config : quiche :: Config , pub disable_client_ip_validation : bool , pub qlog_dir : Option < String > , pub has_gso : bool , pub pacing_offload : bool , pub enable_expensive_packet_count_metrics : bool , pub keylog_file : Option < File > , pub listen_backlog : usize , pub handshake_timeout : Option < Duration > , pub has_ippktinfo : bool , pub has_ipv6pktinfo : bool , }
};
}

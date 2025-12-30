// Generated macro for impl_837 (impl)
macro_rules! Depcrate_settings_quicimpl_837 {
() => {
// Module: crate::settings::quic
// Provides: {"impl_837"}
// Dependencies: {}
impl QuicSettings { # [inline] fn default_alpn () -> Vec < Vec < u8 > > { quiche :: h3 :: APPLICATION_PROTOCOL . iter () . map (| v | v . to_vec ()) . collect () } # [inline] fn default_enable_dgram () -> bool { true } # [inline] fn default_dgram_max_queue_len () -> usize { 65536 } # [inline] fn default_initial_max_data () -> u64 { 10_000_000 } # [inline] fn default_initial_max_stream_data () -> u64 { 1_000_000 } # [inline] fn default_initial_max_streams () -> u64 { 100 } # [inline] fn default_max_idle_timeout () -> Option < Duration > { Some (Duration :: from_secs (56)) } # [inline] fn default_max_recv_udp_payload_size () -> usize { 1350 } # [inline] fn default_max_send_udp_payload_size () -> usize { 1350 } # [inline] fn default_disable_active_migration () -> bool { true } # [inline] fn default_cc_algorithm () -> String { "cubic" . to_string () } # [inline] fn default_initial_congestion_window_packets () -> usize { 10 } # [inline] fn default_enable_hystart () -> bool { true } # [inline] fn default_listen_backlog () -> usize { 1024 } # [inline] fn default_max_connection_window () -> u64 { 24 * 1024 * 1024 } # [inline] fn default_max_stream_window () -> u64 { 16 * 1024 * 1024 } # [inline] fn default_grease () -> bool { true } # [inline] fn default_amplification_factor () -> usize { 3 } # [inline] fn default_send_capacity_factor () -> f64 { 1.0 } # [inline] fn default_ack_delay_exponent () -> u64 { 3 } # [inline] fn default_max_ack_delay () -> u64 { 25 } # [inline] fn default_active_connection_id_limit () -> u64 { 2 } # [inline] fn default_max_path_challenge_recv_queue_len () -> usize { 3 } }
};
}

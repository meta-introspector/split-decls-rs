// Generated macro for quic_expensive_metrics_ip_reduce (function)
macro_rules! Depcrate_metricsquic_expensive_metrics_ip_reduce {
() => {
// Module: crate::metrics
// Provides: {"quic_expensive_metrics_ip_reduce"}
// Dependencies: {}
pub (crate) fn quic_expensive_metrics_ip_reduce (ip : IpAddr) -> Option < IpAddr > { const QUIC_INITIAL_METRICS_V4_PREFIX : u8 = 20 ; const QUIC_INITIAL_METRICS_V6_PREFIX : u8 = 32 ; let prefix = if ip . is_ipv4 () { QUIC_INITIAL_METRICS_V4_PREFIX } else { QUIC_INITIAL_METRICS_V6_PREFIX } ; if let Ok (ip_net) = ipnetwork :: IpNetwork :: new (ip , prefix) { Some (ip_net . network ()) } else { None } }
};
}

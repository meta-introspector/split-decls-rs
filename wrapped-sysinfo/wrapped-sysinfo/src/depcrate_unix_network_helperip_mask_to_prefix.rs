// Generated macro for ip_mask_to_prefix (function)
macro_rules! Depcrate_unix_network_helperip_mask_to_prefix {
() => {
// Module: crate::unix::network_helper
// Provides: {"ip_mask_to_prefix"}
// Dependencies: {}
pub (crate) fn ip_mask_to_prefix (mask : IpAddr) -> Result < u8 , & 'static str > { match mask { IpAddr :: V4 (mask) => ipv4_mask_to_prefix (mask) , IpAddr :: V6 (mask) => ipv6_mask_to_prefix (mask) , } }
};
}

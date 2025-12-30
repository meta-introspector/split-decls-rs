// Generated macro for get_ip_networks (function)
macro_rules! Depcrate_windows_network_helperget_ip_networks {
() => {
// Module: crate::windows::network_helper
// Provides: {"get_ip_networks"}
// Dependencies: {}
fn get_ip_networks (mut prefixes_ptr : * mut IP_ADAPTER_UNICAST_ADDRESS_LH) -> HashSet < IpNetwork > { let mut ip_networks = HashSet :: new () ; while ! prefixes_ptr . is_null () { let prefix = unsafe { prefixes_ptr . read_unaligned () } ; if let Some (socket_address) = NonNull :: new (prefix . Address . lpSockaddr) && let Some (ipaddr) = get_ip_address_from_socket_address (socket_address) { ip_networks . insert (IpNetwork { addr : ipaddr , prefix : prefix . OnLinkPrefixLength , }) ; } prefixes_ptr = prefix . Next ; } ip_networks }
};
}

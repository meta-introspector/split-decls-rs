// Generated macro for get_interface_ip_networks (function)
macro_rules! Depcrate_windows_network_helperget_interface_ip_networks {
() => {
// Module: crate::windows::network_helper
// Provides: {"get_interface_ip_networks"}
// Dependencies: {}
pub (crate) unsafe fn get_interface_ip_networks () -> HashMap < String , HashSet < IpNetwork > > { match unsafe { get_interface_address () } { Ok (mut interface_iter) => interface_iter . generate_ip_networks () , _ => HashMap :: new () , } }
};
}

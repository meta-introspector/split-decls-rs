// Generated macro for impl_1158 (impl)
macro_rules! Depcrate_windows_networkimpl_1158 {
() => {
// Module: crate::windows::network
// Provides: {"impl_1158"}
// Dependencies: {}
impl NetworkDataInner { pub (crate) fn received (& self) -> u64 { self . current_in . saturating_sub (self . old_in) } pub (crate) fn total_received (& self) -> u64 { self . current_in } pub (crate) fn transmitted (& self) -> u64 { self . current_out . saturating_sub (self . old_out) } pub (crate) fn total_transmitted (& self) -> u64 { self . current_out } pub (crate) fn packets_received (& self) -> u64 { self . packets_in . saturating_sub (self . old_packets_in) } pub (crate) fn total_packets_received (& self) -> u64 { self . packets_in } pub (crate) fn packets_transmitted (& self) -> u64 { self . packets_out . saturating_sub (self . old_packets_out) } pub (crate) fn total_packets_transmitted (& self) -> u64 { self . packets_out } pub (crate) fn errors_on_received (& self) -> u64 { self . errors_in . saturating_sub (self . old_errors_in) } pub (crate) fn total_errors_on_received (& self) -> u64 { self . errors_in } pub (crate) fn errors_on_transmitted (& self) -> u64 { self . errors_out . saturating_sub (self . old_errors_out) } pub (crate) fn total_errors_on_transmitted (& self) -> u64 { self . errors_out } pub (crate) fn mac_address (& self) -> MacAddr { self . mac_addr } pub (crate) fn ip_networks (& self) -> & [IpNetwork] { & self . ip_networks } pub (crate) fn mtu (& self) -> u64 { self . mtu } }
};
}

// Generated macro for impl_813 (impl)
macro_rules! Depcrate_unix_linux_networkimpl_813 {
() => {
// Module: crate::unix::linux::network
// Provides: {"impl_813"}
// Dependencies: {}
impl NetworkDataInner { pub (crate) fn received (& self) -> u64 { self . rx_bytes . saturating_sub (self . old_rx_bytes) } pub (crate) fn total_received (& self) -> u64 { self . rx_bytes } pub (crate) fn transmitted (& self) -> u64 { self . tx_bytes . saturating_sub (self . old_tx_bytes) } pub (crate) fn total_transmitted (& self) -> u64 { self . tx_bytes } pub (crate) fn packets_received (& self) -> u64 { self . rx_packets . saturating_sub (self . old_rx_packets) } pub (crate) fn total_packets_received (& self) -> u64 { self . rx_packets } pub (crate) fn packets_transmitted (& self) -> u64 { self . tx_packets . saturating_sub (self . old_tx_packets) } pub (crate) fn total_packets_transmitted (& self) -> u64 { self . tx_packets } pub (crate) fn errors_on_received (& self) -> u64 { self . rx_errors . saturating_sub (self . old_rx_errors) } pub (crate) fn total_errors_on_received (& self) -> u64 { self . rx_errors } pub (crate) fn errors_on_transmitted (& self) -> u64 { self . tx_errors . saturating_sub (self . old_tx_errors) } pub (crate) fn total_errors_on_transmitted (& self) -> u64 { self . tx_errors } pub (crate) fn mac_address (& self) -> MacAddr { self . mac_addr } pub (crate) fn ip_networks (& self) -> & [IpNetwork] { & self . ip_networks } pub (crate) fn mtu (& self) -> u64 { self . mtu } }
};
}

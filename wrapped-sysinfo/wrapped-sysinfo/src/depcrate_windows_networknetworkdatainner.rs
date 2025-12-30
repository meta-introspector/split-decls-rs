// Generated macro for NetworkDataInner (struct)
macro_rules! Depcrate_windows_networkNetworkDataInner {
() => {
// Module: crate::windows::network
// Provides: {"NetworkDataInner"}
// Dependencies: {}
pub (crate) struct NetworkDataInner { current_out : u64 , old_out : u64 , current_in : u64 , old_in : u64 , packets_in : u64 , old_packets_in : u64 , packets_out : u64 , old_packets_out : u64 , errors_in : u64 , old_errors_in : u64 , errors_out : u64 , old_errors_out : u64 , updated : bool , pub (crate) mac_addr : MacAddr , pub (crate) ip_networks : Vec < IpNetwork > , # [doc = " Interface Maximum Transfer Unit (MTU)"] mtu : u64 , }
};
}

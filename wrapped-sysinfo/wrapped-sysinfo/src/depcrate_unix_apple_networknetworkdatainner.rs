// Generated macro for NetworkDataInner (struct)
macro_rules! Depcrate_unix_apple_networkNetworkDataInner {
() => {
// Module: crate::unix::apple::network
// Provides: {"NetworkDataInner"}
// Dependencies: {}
# [derive (PartialEq , Eq)] pub (crate) struct NetworkDataInner { current_in : u64 , old_in : u64 , current_out : u64 , old_out : u64 , packets_in : u64 , old_packets_in : u64 , packets_out : u64 , old_packets_out : u64 , errors_in : u64 , old_errors_in : u64 , errors_out : u64 , old_errors_out : u64 , updated : bool , # [doc = " MAC address"] pub (crate) mac_addr : MacAddr , # [doc = " IP networks"] pub (crate) ip_networks : Vec < IpNetwork > , # [doc = " Interface Maximum Transfer Unit (MTU)"] mtu : u64 , }
};
}

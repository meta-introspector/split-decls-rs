// Generated macro for NetworkDataInner (struct)
macro_rules! Depcrate_unix_linux_networkNetworkDataInner {
() => {
// Module: crate::unix::linux::network
// Provides: {"NetworkDataInner"}
// Dependencies: {}
pub (crate) struct NetworkDataInner { # [doc = " Total number of bytes received over interface."] rx_bytes : u64 , old_rx_bytes : u64 , # [doc = " Total number of bytes transmitted over interface."] tx_bytes : u64 , old_tx_bytes : u64 , # [doc = " Total number of packets received."] rx_packets : u64 , old_rx_packets : u64 , # [doc = " Total number of packets transmitted."] tx_packets : u64 , old_tx_packets : u64 , # [doc = " Shows the total number of packets received with error. This includes"] # [doc = " too-long-frames errors, ring-buffer overflow errors, CRC errors,"] # [doc = " frame alignment errors, fifo overruns, and missed packets."] rx_errors : u64 , old_rx_errors : u64 , # [doc = " similar to `rx_errors`"] tx_errors : u64 , old_tx_errors : u64 , # [doc = " MAC address"] pub (crate) mac_addr : MacAddr , pub (crate) ip_networks : Vec < IpNetwork > , # [doc = " Interface Maximum Transfer Unit (MTU)"] mtu : u64 , # [doc = " Whether or not the above data has been updated during refresh"] updated : bool , }
};
}

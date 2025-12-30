// Generated macro for NetworkDataInner (struct)
macro_rules! Depcrate_unix_freebsd_networkNetworkDataInner {
() => {
// Module: crate::unix::freebsd::network
// Provides: {"NetworkDataInner"}
// Dependencies: {}
pub (crate) struct NetworkDataInner { # [doc = " Total number of bytes received over interface."] ifi_ibytes : u64 , old_ifi_ibytes : u64 , # [doc = " Total number of bytes transmitted over interface."] ifi_obytes : u64 , old_ifi_obytes : u64 , # [doc = " Total number of packets received."] ifi_ipackets : u64 , old_ifi_ipackets : u64 , # [doc = " Total number of packets transmitted."] ifi_opackets : u64 , old_ifi_opackets : u64 , # [doc = " Shows the total number of packets received with error. This includes"] # [doc = " too-long-frames errors, ring-buffer overflow errors, CRC errors,"] # [doc = " frame alignment errors, fifo overruns, and missed packets."] ifi_ierrors : u64 , old_ifi_ierrors : u64 , # [doc = " similar to `ifi_ierrors`"] ifi_oerrors : u64 , old_ifi_oerrors : u64 , # [doc = " Whether or not the above data has been updated during refresh"] updated : bool , # [doc = " MAC address"] pub (crate) mac_addr : MacAddr , # [doc = " IP networks"] pub (crate) ip_networks : Vec < IpNetwork > , # [doc = " Interface Maximum Transfer Unit (MTU)"] mtu : u64 , }
};
}

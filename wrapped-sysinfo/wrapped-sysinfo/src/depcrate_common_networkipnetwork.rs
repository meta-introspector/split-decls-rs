// Generated macro for IpNetwork (struct)
macro_rules! Depcrate_common_networkIpNetwork {
() => {
// Module: crate::common::network
// Provides: {"IpNetwork"}
// Dependencies: {}
# [doc = " IP networks address for network interface."] # [doc = ""] # [doc = " It is returned by [`NetworkData::ip_networks`][crate::NetworkData::ip_networks]."] # [derive (Debug , Clone , Copy , Hash , PartialEq , Eq , PartialOrd , Ord)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct IpNetwork { # [doc = " The IP of the network interface."] pub addr : IpAddr , # [doc = " The netmask, prefix of the IP address."] pub prefix : u8 , }
};
}

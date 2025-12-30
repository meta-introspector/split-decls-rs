// Generated macro for MacAddr (struct)
macro_rules! Depcrate_common_networkMacAddr {
() => {
// Module: crate::common::network
// Provides: {"MacAddr"}
// Dependencies: {}
# [doc = " MAC address for network interface."] # [doc = ""] # [doc = " It is returned by [`NetworkData::mac_address`][crate::NetworkData::mac_address]."] # [derive (Debug , Clone , Copy , Hash , PartialEq , Eq , PartialOrd , Ord)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct MacAddr (pub [u8 ; 6]) ;
};
}

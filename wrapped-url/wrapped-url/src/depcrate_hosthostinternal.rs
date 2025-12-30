// Generated macro for HostInternal (enum)
macro_rules! Depcrate_hostHostInternal {
() => {
// Module: crate::host
// Provides: {"HostInternal"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub (crate) enum HostInternal { None , Domain , Ipv4 (Ipv4Addr) , Ipv6 (Ipv6Addr) , }
};
}

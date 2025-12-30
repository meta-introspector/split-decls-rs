// Generated macro for other_5684 (other)
macro_rules! Depcrate_shared_netioapiother_5684 {
() => {
// Module: crate::shared::netioapi
// Provides: {"other_5684"}
// Dependencies: {}
extern "system" { pub fn NotifyStableUnicastIpAddressTable (Family : ADDRESS_FAMILY , Table : * mut PMIB_UNICASTIPADDRESS_TABLE , CallerCallback : PSTABLE_UNICAST_IPADDRESS_TABLE_CALLBACK , CallerContext : PVOID , NotificationHandle : * mut HANDLE ,) -> NETIOAPI_API ; pub fn SetUnicastIpAddressEntry (Row : * const MIB_UNICASTIPADDRESS_ROW ,) -> NETIOAPI_API ; }
};
}

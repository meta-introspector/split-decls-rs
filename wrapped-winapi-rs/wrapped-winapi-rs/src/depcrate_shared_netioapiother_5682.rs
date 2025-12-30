// Generated macro for other_5682 (other)
macro_rules! Depcrate_shared_netioapiother_5682 {
() => {
// Module: crate::shared::netioapi
// Provides: {"other_5682"}
// Dependencies: {}
extern "system" { pub fn CreateUnicastIpAddressEntry (Row : * const MIB_UNICASTIPADDRESS_ROW ,) -> NETIOAPI_API ; pub fn DeleteUnicastIpAddressEntry (Row : * const MIB_UNICASTIPADDRESS_ROW ,) -> NETIOAPI_API ; pub fn GetUnicastIpAddressEntry (Row : PMIB_UNICASTIPADDRESS_ROW) -> NETIOAPI_API ; pub fn GetUnicastIpAddressTable (Family : ADDRESS_FAMILY , Table : * mut PMIB_UNICASTIPADDRESS_TABLE ,) -> NETIOAPI_API ; pub fn InitializeUnicastIpAddressEntry (Row : PMIB_UNICASTIPADDRESS_ROW ,) ; pub fn NotifyUnicastIpAddressChange (Family : ADDRESS_FAMILY , Callback : PUNICAST_IPADDRESS_CHANGE_CALLBACK , CallerContext : PVOID , InitialNotification : BOOLEAN , NotificationHandle : * mut HANDLE ,) -> NETIOAPI_API ; }
};
}

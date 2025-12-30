// Generated macro for other_5676 (other)
macro_rules! Depcrate_shared_netioapiother_5676 {
() => {
// Module: crate::shared::netioapi
// Provides: {"other_5676"}
// Dependencies: {}
extern "system" { pub fn GetIfStackTable (Table : * mut PMIB_IFSTACK_TABLE ,) -> NETIOAPI_API ; pub fn GetInvertedIfStackTable (Table : * mut PMIB_INVERTEDIFSTACK_TABLE ,) -> NETIOAPI_API ; pub fn GetIpInterfaceEntry (Row : PMIB_IPINTERFACE_ROW ,) -> NETIOAPI_API ; pub fn GetIpInterfaceTable (Family : ADDRESS_FAMILY , Table : * mut PMIB_IPINTERFACE_TABLE ,) -> NETIOAPI_API ; pub fn InitializeIpInterfaceEntry (Row : PMIB_IPINTERFACE_ROW ,) ; pub fn NotifyIpInterfaceChange (Family : ADDRESS_FAMILY , Callback : PIPINTERFACE_CHANGE_CALLBACK , CallerContext : PVOID , InitialNotification : BOOLEAN , NotificationHandle : * mut HANDLE) -> NETIOAPI_API ; pub fn SetIpInterfaceEntry (Row : PMIB_IPINTERFACE_ROW ,) -> NETIOAPI_API ; pub fn GetIpNetworkConnectionBandwidthEstimates (InterfaceIndex : NET_IFINDEX , AddressFamily : ADDRESS_FAMILY , BandwidthEstimates : PMIB_IP_NETWORK_CONNECTION_BANDWIDTH_ESTIMATES ,) -> NETIOAPI_API ; }
};
}

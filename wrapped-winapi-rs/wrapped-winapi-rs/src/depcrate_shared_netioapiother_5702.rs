// Generated macro for other_5702 (other)
macro_rules! Depcrate_shared_netioapiother_5702 {
() => {
// Module: crate::shared::netioapi
// Provides: {"other_5702"}
// Dependencies: {}
extern "system" { pub fn CreateIpForwardEntry2 (Row : * const MIB_IPFORWARD_ROW2 ,) -> NETIOAPI_API ; pub fn DeleteIpForwardEntry2 (Row : * const MIB_IPFORWARD_ROW2 ,) -> NETIOAPI_API ; pub fn GetBestRoute2 (InterfaceLuid : * mut NET_LUID , InterfaceIndex : NET_IFINDEX , SourceAddress : * const SOCKADDR_INET , DestinationAddress : * const SOCKADDR_INET , AddressSortOptions : ULONG , BestRoute : PMIB_IPFORWARD_ROW2 , BestSourceAddress : * mut SOCKADDR_INET ,) -> NETIOAPI_API ; pub fn GetIpForwardEntry2 (Row : PMIB_IPFORWARD_ROW2 ,) -> NETIOAPI_API ; pub fn GetIpForwardTable2 (Family : ADDRESS_FAMILY , Table : * mut PMIB_IPFORWARD_TABLE2 ,) -> NETIOAPI_API ; pub fn InitializeIpForwardEntry (Row : PMIB_IPFORWARD_ROW2 ,) ; pub fn NotifyRouteChange2 (AddressFamily : ADDRESS_FAMILY , Callback : PIPFORWARD_CHANGE_CALLBACK , CallerContext : PVOID , InitialNotification : BOOLEAN , NotificationHandle : * mut HANDLE ,) -> NETIOAPI_API ; pub fn SetIpForwardEntry2 (Route : * const MIB_IPFORWARD_ROW2 ,) -> NETIOAPI_API ; }
};
}

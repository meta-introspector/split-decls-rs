// Generated macro for other_5716 (other)
macro_rules! Depcrate_shared_netioapiother_5716 {
() => {
// Module: crate::shared::netioapi
// Provides: {"other_5716"}
// Dependencies: {}
extern "system" { pub fn CreateIpNetEntry2 (Row : * const MIB_IPNET_ROW2 ,) -> NETIOAPI_API ; pub fn DeleteIpNetEntry2 (Row : * const MIB_IPNET_ROW2 ,) -> NETIOAPI_API ; pub fn FlushIpNetTable2 (Family : ADDRESS_FAMILY , InterfaceIndex : NET_IFINDEX ,) -> NETIOAPI_API ; pub fn GetIpNetEntry2 (Row : PMIB_IPNET_ROW2 ,) -> NETIOAPI_API ; pub fn GetIpNetTable2 (Family : ADDRESS_FAMILY , Table : * mut PMIB_IPNET_TABLE2 ,) -> NETIOAPI_API ; pub fn ResolveIpNetEntry2 (Row : PMIB_IPNET_ROW2 , SourceAddress : * const SOCKADDR_INET ,) -> NETIOAPI_API ; pub fn SetIpNetEntry2 (Row : PMIB_IPNET_ROW2 ,) -> NETIOAPI_API ; }
};
}

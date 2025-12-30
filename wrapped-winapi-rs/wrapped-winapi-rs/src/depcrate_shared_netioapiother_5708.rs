// Generated macro for other_5708 (other)
macro_rules! Depcrate_shared_netioapiother_5708 {
() => {
// Module: crate::shared::netioapi
// Provides: {"other_5708"}
// Dependencies: {}
extern "system" { pub fn FlushIpPathTable (Family : ADDRESS_FAMILY ,) -> NETIOAPI_API ; pub fn GetIpPathEntry (Row : PMIB_IPPATH_ROW ,) -> NETIOAPI_API ; pub fn GetIpPathTable (Family : ADDRESS_FAMILY , Table : * mut PMIB_IPPATH_TABLE ,) -> NETIOAPI_API ; }
};
}

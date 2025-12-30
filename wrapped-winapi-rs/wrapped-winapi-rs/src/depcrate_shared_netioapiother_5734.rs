// Generated macro for other_5734 (other)
macro_rules! Depcrate_shared_netioapiother_5734 {
() => {
// Module: crate::shared::netioapi
// Provides: {"other_5734"}
// Dependencies: {}
extern "system" { pub fn GetDnsSettings (Settings : * mut DNS_SETTINGS ,) -> NETIOAPI_API ; pub fn FreeDnsSettings (Settings : * mut DNS_SETTINGS ,) ; pub fn SetDnsSettings (Settings : * const DNS_SETTINGS ,) -> NETIOAPI_API ; pub fn GetInterfaceDnsSettings (Interface : GUID , Settings : * mut DNS_INTERFACE_SETTINGS ,) -> NETIOAPI_API ; pub fn FreeInterfaceDnsSettings (Settings : * mut DNS_INTERFACE_SETTINGS ,) ; pub fn SetInterfaceDnsSettings (Interface : GUID , Settings : * const DNS_INTERFACE_SETTINGS ,) -> NETIOAPI_API ; }
};
}

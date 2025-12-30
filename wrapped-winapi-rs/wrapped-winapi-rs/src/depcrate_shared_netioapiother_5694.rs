// Generated macro for other_5694 (other)
macro_rules! Depcrate_shared_netioapiother_5694 {
() => {
// Module: crate::shared::netioapi
// Provides: {"other_5694"}
// Dependencies: {}
extern "system" { pub fn GetMulticastIpAddressEntry (Row : PMIB_MULTICASTIPADDRESS_ROW ,) -> NETIOAPI_API ; pub fn GetMulticastIpAddressTable (Family : ADDRESS_FAMILY , Table : * mut PMIB_MULTICASTIPADDRESS_TABLE ,) -> NETIOAPI_API ; }
};
}

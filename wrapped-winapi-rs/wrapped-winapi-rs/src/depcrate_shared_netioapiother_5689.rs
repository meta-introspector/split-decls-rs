// Generated macro for other_5689 (other)
macro_rules! Depcrate_shared_netioapiother_5689 {
() => {
// Module: crate::shared::netioapi
// Provides: {"other_5689"}
// Dependencies: {}
extern "system" { pub fn CreateAnycastIpAddressEntry (Row : * const MIB_ANYCASTIPADDRESS_ROW ,) -> NETIOAPI_API ; pub fn DeleteAnycastIpAddressEntry (Row : * const MIB_ANYCASTIPADDRESS_ROW ,) -> NETIOAPI_API ; pub fn GetAnycastIpAddressEntry (Row : PMIB_ANYCASTIPADDRESS_ROW ,) -> NETIOAPI_API ; pub fn GetAnycastIpAddressTable (Family : ADDRESS_FAMILY , Table : * mut PMIB_ANYCASTIPADDRESS_TABLE ,) -> NETIOAPI_API ; }
};
}

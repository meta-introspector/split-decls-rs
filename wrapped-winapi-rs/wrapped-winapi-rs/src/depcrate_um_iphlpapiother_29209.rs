// Generated macro for other_29209 (other)
macro_rules! Depcrate_um_iphlpapiother_29209 {
() => {
// Module: crate::um::iphlpapi
// Provides: {"other_29209"}
// Dependencies: {}
extern "system" { pub fn ParseNetworkString (NetworkString : * const * mut WCHAR , Types : DWORD , AddressInfo : PNET_ADDRESS_INFO , PortNumber : * mut USHORT , PrefixLength : * mut BYTE ,) -> DWORD ; }
};
}

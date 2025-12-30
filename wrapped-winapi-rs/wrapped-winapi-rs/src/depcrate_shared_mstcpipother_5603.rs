// Generated macro for other_5603 (other)
macro_rules! Depcrate_shared_mstcpipother_5603 {
() => {
// Module: crate::shared::mstcpip
// Provides: {"other_5603"}
// Dependencies: {}
extern "system" { pub fn RtlEthernetAddressToStringA (Addr : * const DL_EUI48 , S : PSTR ,) -> PSTR ; pub fn RtlEthernetAddressToStringW (Addr : * const DL_EUI48 , S : PWSTR ,) -> PWSTR ; pub fn RtlEthernetStringToAddressA (S : PCSTR , Terminator : * mut PCSTR , Addr : * mut DL_EUI48 ,) -> LONG ; pub fn RtlEthernetStringToAddressW (S : PCWSTR , Terminator : * mut LPCWSTR , Addr : * mut DL_EUI48 ,) -> LONG ; }
};
}

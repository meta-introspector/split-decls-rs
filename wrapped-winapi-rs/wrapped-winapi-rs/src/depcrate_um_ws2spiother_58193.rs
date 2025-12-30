// Generated macro for other_58193 (other)
macro_rules! Depcrate_um_ws2spiother_58193 {
() => {
// Module: crate::um::ws2spi
// Provides: {"other_58193"}
// Dependencies: {}
extern "system" { pub fn WSAAdvertiseProvider (puuidProviderId : * const GUID , pNSPv2Routine : * const LPCNSPV2_ROUTINE ,) -> INT ; pub fn WSAUnadvertiseProvider (puuidProviderId : * const GUID ,) -> INT ; pub fn WSAProviderCompleteAsyncCall (hAsyncCall : HANDLE , iRetCode : INT ,) -> INT ; }
};
}

// Generated macro for other_57729 (other)
macro_rules! Depcrate_um_wlanapiother_57729 {
() => {
// Module: crate::um::wlanapi
// Provides: {"other_57729"}
// Dependencies: {}
extern "system" { pub fn WlanHostedNetworkQueryStatus (hClientHandle : HANDLE , ppWlanHostedNetworkStatus : * mut PWLAN_HOSTED_NETWORK_STATUS , pvReserved : PVOID ,) -> DWORD ; pub fn WlanHostedNetworkSetSecondaryKey (hClientHandle : HANDLE , dwKeyLength : DWORD , pucKeyData : PUCHAR , bIsPassPhrase : BOOL , bPersistent : BOOL , pFailReason : PWLAN_HOSTED_NETWORK_REASON , pvReserved : PVOID ,) -> DWORD ; pub fn WlanHostedNetworkQuerySecondaryKey (hClientHandle : HANDLE , pdwKeyLength : PDWORD , ppucKeyData : * mut PUCHAR , pbIsPassPhrase : PBOOL , pbPersistent : PBOOL , pFailReason : PWLAN_HOSTED_NETWORK_REASON , pvReserved : PVOID ,) -> DWORD ; pub fn WlanRegisterVirtualStationNotification (hClientHandle : HANDLE , bRegister : BOOL , pReserved : PVOID ,) -> DWORD ; }
};
}

// Generated macro for other_57709 (other)
macro_rules! Depcrate_um_wlanapiother_57709 {
() => {
// Module: crate::um::wlanapi
// Provides: {"other_57709"}
// Dependencies: {}
extern "system" { pub fn WlanHostedNetworkStartUsing (hClientHandle : HANDLE , pFailReason : PWLAN_HOSTED_NETWORK_REASON , pvReserved : PVOID ,) -> DWORD ; pub fn WlanHostedNetworkStopUsing (hClientHandle : HANDLE , pFailReason : PWLAN_HOSTED_NETWORK_REASON , pvReserved : PVOID ,) -> DWORD ; pub fn WlanHostedNetworkForceStart (hClientHandle : HANDLE , pFailReason : PWLAN_HOSTED_NETWORK_REASON , pvReserved : PVOID ,) -> DWORD ; pub fn WlanHostedNetworkForceStop (hClientHandle : HANDLE , pFailReason : PWLAN_HOSTED_NETWORK_REASON , pvReserved : PVOID ,) -> DWORD ; }
};
}

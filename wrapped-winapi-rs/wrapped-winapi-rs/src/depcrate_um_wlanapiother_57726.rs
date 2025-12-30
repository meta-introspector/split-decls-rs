// Generated macro for other_57726 (other)
macro_rules! Depcrate_um_wlanapiother_57726 {
() => {
// Module: crate::um::wlanapi
// Provides: {"other_57726"}
// Dependencies: {}
extern "system" { pub fn WlanHostedNetworkQueryProperty (hClientHandle : HANDLE , OpCode : WLAN_HOSTED_NETWORK_OPCODE , pdwDataSize : PDWORD , ppvData : * mut PVOID , pWlanOpcodeValueType : PWLAN_OPCODE_VALUE_TYPE , pvReserved : PVOID ,) -> DWORD ; pub fn WlanHostedNetworkSetProperty (hClientHandle : HANDLE , OpCode : WLAN_HOSTED_NETWORK_OPCODE , dwDataSize : DWORD , pvData : PVOID , pFailReason : PWLAN_HOSTED_NETWORK_REASON , pvReserved : PVOID ,) -> DWORD ; pub fn WlanHostedNetworkInitSettings (hClientHandle : HANDLE , pFailReason : PWLAN_HOSTED_NETWORK_REASON , pvReserved : PVOID ,) -> DWORD ; pub fn WlanHostedNetworkRefreshSecuritySettings (hClientHandle : HANDLE , pFailReason : PWLAN_HOSTED_NETWORK_REASON , pvReserved : PVOID ,) -> DWORD ; }
};
}

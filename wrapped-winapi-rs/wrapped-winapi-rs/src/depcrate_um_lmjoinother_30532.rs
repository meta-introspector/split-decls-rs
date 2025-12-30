// Generated macro for other_30532 (other)
macro_rules! Depcrate_um_lmjoinother_30532 {
() => {
// Module: crate::um::lmjoin
// Provides: {"other_30532"}
// Dependencies: {}
extern "system" { pub fn NetCreateProvisioningPackage (pProvisioningParams : PNETSETUP_PROVISIONING_PARAMS , ppPackageBinData : * mut PBYTE , pdwPackageBinDataSize : * mut DWORD , ppPackageTextData : * mut LPWSTR ,) -> NET_API_STATUS ; pub fn NetRequestProvisioningPackageInstall (pPackageBinData : * mut BYTE , dwPackageBinDataSize : DWORD , dwProvisionOptions : DWORD , lpWindowsPath : LPCWSTR , pvReserved : PVOID ,) -> NET_API_STATUS ; pub fn NetGetAadJoinInformation (pcszTenantId : LPCWSTR , ppJoinInfo : * mut PDSREG_JOIN_INFO ,) -> HRESULT ; pub fn NetFreeAadJoinInformation (pJoinInfo : PDSREG_JOIN_INFO ,) ; }
};
}

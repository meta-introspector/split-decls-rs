// Generated macro for other_30527 (other)
macro_rules! Depcrate_um_lmjoinother_30527 {
() => {
// Module: crate::um::lmjoin
// Provides: {"other_30527"}
// Dependencies: {}
extern "system" { pub fn NetProvisionComputerAccount (lpDomain : LPCWSTR , lpMachineName : LPCWSTR , lpMachineAccountOU : LPCWSTR , lpDcName : LPCWSTR , dwOptions : DWORD , pProvisionBinData : * mut PBYTE , pdwProvisionBinDataSize : * mut DWORD , pProvisionTextData : * mut LPWSTR ,) -> NET_API_STATUS ; pub fn NetRequestOfflineDomainJoin (pProvisionBinData : * mut BYTE , cbProvisionBinDataSize : DWORD , dwOptions : DWORD , lpWindowsPath : LPCWSTR ,) -> NET_API_STATUS ; }
};
}

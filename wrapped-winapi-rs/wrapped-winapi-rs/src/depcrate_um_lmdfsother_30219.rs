// Generated macro for other_30219 (other)
macro_rules! Depcrate_um_lmdfsother_30219 {
() => {
// Module: crate::um::lmdfs
// Provides: {"other_30219"}
// Dependencies: {}
extern "system" { pub fn NetDfsRemoveRootTarget (pDfsPath : LPWSTR , pTargetPath : LPWSTR , Flags : DWORD ,) -> NET_API_STATUS ; pub fn NetDfsGetSecurity (DfsEntryPath : LPWSTR , SecurityInformation : SECURITY_INFORMATION , ppSecurityDescriptor : * mut PSECURITY_DESCRIPTOR , lpcbSecurityDescriptor : LPDWORD ,) -> NET_API_STATUS ; pub fn NetDfsSetSecurity (DfsEntryPath : LPWSTR , SecurityInformation : SECURITY_INFORMATION , pSecurityDescriptor : PSECURITY_DESCRIPTOR ,) -> NET_API_STATUS ; pub fn NetDfsGetStdContainerSecurity (MachineName : LPWSTR , SecurityInformation : SECURITY_INFORMATION , ppSecurityDescriptor : * mut PSECURITY_DESCRIPTOR , lpcbSecurityDescriptor : LPDWORD ,) -> NET_API_STATUS ; pub fn NetDfsSetStdContainerSecurity (MachineName : LPWSTR , SecurityInformation : SECURITY_INFORMATION , pSecurityDescriptor : PSECURITY_DESCRIPTOR ,) -> NET_API_STATUS ; pub fn NetDfsGetFtContainerSecurity (DomainName : LPWSTR , SecurityInformation : SECURITY_INFORMATION , ppSecurityDescriptor : * mut PSECURITY_DESCRIPTOR , lpcbSecurityDescriptor : LPDWORD ,) -> NET_API_STATUS ; pub fn NetDfsSetFtContainerSecurity (DomainName : LPWSTR , SecurityInformation : SECURITY_INFORMATION , pSecurityDescriptor : PSECURITY_DESCRIPTOR ,) -> NET_API_STATUS ; }
};
}

// Generated macro for other_19784 (other)
macro_rules! Depcrate_um_appmgmtother_19784 {
() => {
// Module: crate::um::appmgmt
// Provides: {"other_19784"}
// Dependencies: {}
extern "system" { pub fn InstallApplication (pInstallInfo : PINSTALLDATA ,) -> DWORD ; pub fn UninstallApplication (ProductCode : LPWSTR , dwStatus : DWORD ,) -> DWORD ; pub fn CommandLineFromMsiDescriptor (Descriptor : LPWSTR , CommandLine : LPWSTR , CommandLineLength : * mut DWORD ,) -> DWORD ; pub fn GetManagedApplications (pCategory : * mut GUID , dwQueryFlags : DWORD , dwInfoLevel : DWORD , pdwApps : LPDWORD , prgManagedApps : * mut PMANAGEDAPPLICATION ,) -> DWORD ; pub fn GetLocalManagedApplications (bUserApps : BOOL , pdwApps : LPDWORD , prgManagedApps : * mut PMANAGEDAPPLICATION ,) -> DWORD ; pub fn GetLocalManagedApplicationData (ProductCode : LPWSTR , DisplayName : * mut LPWSTR , SupportUrl : * mut LPWSTR ,) ; pub fn GetManagedApplicationCategories (dwReserved : DWORD , pAppCategory : * mut APPCATEGORYINFOLIST ,) -> DWORD ; }
};
}

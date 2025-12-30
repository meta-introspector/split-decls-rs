// Generated macro for other_36275 (other)
macro_rules! Depcrate_um_restartmanagerother_36275 {
() => {
// Module: crate::um::restartmanager
// Provides: {"other_36275"}
// Dependencies: {}
extern "system" { pub fn RmStartSession (pSessionHandle : * mut DWORD , dwSessionFlags : DWORD , strSessionKey : * mut WCHAR ,) -> DWORD ; pub fn RmJoinSession (pSessionHandle : * mut DWORD , strSessionKey : * const WCHAR ,) -> DWORD ; pub fn RmEndSession (dwSessionHandle : DWORD ,) -> DWORD ; pub fn RmRegisterResources (dwSessionHandle : DWORD , nFiles : UINT , rgsFileNames : * mut LPCWSTR , nApplications : UINT , rgApplications : * mut RM_UNIQUE_PROCESS , nServices : UINT , rgsServiceNames : * mut LPCWSTR ,) -> DWORD ; pub fn RmGetList (dwSessionHandle : DWORD , pnProcInfoNeeded : * mut UINT , pnProcInfo : * mut UINT , rgAffectedApps : * mut RM_PROCESS_INFO , lpdwRebootReasons : LPDWORD ,) -> DWORD ; pub fn RmShutdown (dwSessionHandle : DWORD , lActionFlags : ULONG , fnStatus : RM_WRITE_STATUS_CALLBACK ,) -> DWORD ; pub fn RmRestart (dwSessionHandle : DWORD , dwRestartFlags : DWORD , fnStatus : RM_WRITE_STATUS_CALLBACK ,) -> DWORD ; pub fn RmCancelCurrentTask (dwSessionHandle : DWORD ,) -> DWORD ; pub fn RmAddFilter (dwSessionHandle : DWORD , strModuleName : LPCWSTR , pProcess : * mut RM_UNIQUE_PROCESS , strServiceShortName : LPCWSTR , FilterAction : RM_FILTER_ACTION ,) -> DWORD ; pub fn RmRemoveFilter (dwSessionHandle : DWORD , strModuleName : LPCWSTR , pProcess : * mut RM_UNIQUE_PROCESS , strServiceShortName : LPCWSTR ,) -> DWORD ; pub fn RmGetFilterList (dwSessionHandle : DWORD , pbFilterBuf : PBYTE , cbFilterBuf : DWORD , cbFilterBufNeeded : LPDWORD ,) -> DWORD ; }
};
}

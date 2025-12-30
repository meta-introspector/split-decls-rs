// Generated macro for other_40113 (other)
macro_rules! Depcrate_um_winbaseother_40113 {
() => {
// Module: crate::um::winbase
// Provides: {"other_40113"}
// Dependencies: {}
extern "system" { pub fn GetSystemDEPPolicy () -> DEP_SYSTEM_POLICY_TYPE ; pub fn GetSystemRegistryQuota (pdwQuotaAllowed : PDWORD , pdwQuotaUsed : PDWORD ,) -> BOOL ; pub fn FileTimeToDosDateTime (lpFileTime : * const FILETIME , lpFatDate : LPWORD , lpFatTime : LPWORD ,) -> BOOL ; pub fn DosDateTimeToFileTime (wFatDate : WORD , wFatTime : WORD , lpFileTime : LPFILETIME ,) -> BOOL ; pub fn FormatMessageA (dwFlags : DWORD , lpSource : LPCVOID , dwMessageId : DWORD , dwLanguageId : DWORD , lpBuffer : LPSTR , nSize : DWORD , Arguments : * mut va_list ,) -> DWORD ; pub fn FormatMessageW (dwFlags : DWORD , lpSource : LPCVOID , dwMessageId : DWORD , dwLanguageId : DWORD , lpBuffer : LPWSTR , nSize : DWORD , Arguments : * mut va_list ,) -> DWORD ; }
};
}

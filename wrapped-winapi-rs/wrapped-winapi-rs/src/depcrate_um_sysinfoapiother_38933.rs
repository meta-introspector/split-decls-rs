// Generated macro for other_38933 (other)
macro_rules! Depcrate_um_sysinfoapiother_38933 {
() => {
// Module: crate::um::sysinfoapi
// Provides: {"other_38933"}
// Dependencies: {}
extern "system" { pub fn GlobalMemoryStatusEx (lpBuffer : LPMEMORYSTATUSEX ,) -> BOOL ; pub fn GetSystemInfo (lpSystemInfo : LPSYSTEM_INFO ,) ; pub fn GetSystemTime (lpSystemTime : LPSYSTEMTIME ,) ; pub fn GetSystemTimeAsFileTime (lpSystemTimeAsFileTime : LPFILETIME ,) ; pub fn GetLocalTime (lpSystemTime : LPSYSTEMTIME ,) ; pub fn GetVersion () -> DWORD ; pub fn SetLocalTime (lpSystemTime : * const SYSTEMTIME ,) -> BOOL ; pub fn GetTickCount () -> DWORD ; pub fn GetTickCount64 () -> ULONGLONG ; pub fn GetSystemTimeAdjustment (lpTimeAdjustment : PDWORD , lpTimeIncrement : PDWORD , lpTimeAdjustmentDisabled : PBOOL ,) -> BOOL ; pub fn GetSystemDirectoryA (lpBuffer : LPSTR , uSize : UINT ,) -> UINT ; pub fn GetSystemDirectoryW (lpBuffer : LPWSTR , uSize : UINT ,) -> UINT ; pub fn GetWindowsDirectoryA (lpBuffer : LPSTR , uSize : UINT ,) -> UINT ; pub fn GetWindowsDirectoryW (lpBuffer : LPWSTR , uSize : UINT ,) -> UINT ; pub fn GetSystemWindowsDirectoryA (lpBuffer : LPSTR , uSize : UINT ,) -> UINT ; pub fn GetSystemWindowsDirectoryW (lpBuffer : LPWSTR , uSize : UINT ,) -> UINT ; }
};
}

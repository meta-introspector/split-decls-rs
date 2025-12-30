// Generated macro for other_40108 (other)
macro_rules! Depcrate_um_winbaseother_40108 {
() => {
// Module: crate::um::winbase
// Provides: {"other_40108"}
// Dependencies: {}
extern "system" { pub fn Wow64GetThreadContext (hThread : HANDLE , lpContext : PWOW64_CONTEXT ,) -> BOOL ; pub fn Wow64SetThreadContext (hThread : HANDLE , lpContext : * const WOW64_CONTEXT ,) -> BOOL ; pub fn Wow64GetThreadSelectorEntry (hThread : HANDLE , dwSelector : DWORD , lpSelectorEntry : PWOW64_LDT_ENTRY ,) -> BOOL ; pub fn Wow64SuspendThread (hThread : HANDLE ,) -> DWORD ; pub fn DebugSetProcessKillOnExit (KillOnExit : BOOL ,) -> BOOL ; pub fn DebugBreakProcess (Process : HANDLE ,) -> BOOL ; pub fn PulseEvent (hEvent : HANDLE ,) -> BOOL ; pub fn GlobalDeleteAtom (nAtom : ATOM ,) -> ATOM ; pub fn InitAtomTable (nSize : DWORD ,) -> BOOL ; pub fn DeleteAtom (nAtom : ATOM ,) -> ATOM ; pub fn SetHandleCount (uNumber : UINT ,) -> UINT ; pub fn RequestDeviceWakeup (hDevice : HANDLE ,) -> BOOL ; pub fn CancelDeviceWakeupRequest (hDevice : HANDLE ,) -> BOOL ; pub fn GetDevicePowerState (hDevice : HANDLE , pfOn : * mut BOOL ,) -> BOOL ; pub fn SetMessageWaitingIndicator (hMsgIndicator : HANDLE , ulMsgCount : ULONG ,) -> BOOL ; pub fn SetFileShortNameA (hFile : HANDLE , lpShortName : LPCSTR ,) -> BOOL ; pub fn SetFileShortNameW (hFile : HANDLE , lpShortName : LPCWSTR ,) -> BOOL ; }
};
}

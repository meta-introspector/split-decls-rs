// Generated macro for other_57901 (other)
macro_rules! Depcrate_um_wow64apisetother_57901 {
() => {
// Module: crate::um::wow64apiset
// Provides: {"other_57901"}
// Dependencies: {}
extern "system" { pub fn Wow64DisableWow64FsRedirection (OldValue : * mut PVOID ,) -> BOOL ; pub fn Wow64RevertWow64FsRedirection (OlValue : PVOID ,) -> BOOL ; pub fn IsWow64Process (hProcess : HANDLE , Wow64Process : PBOOL ,) -> BOOL ; pub fn GetSystemWow64DirectoryA (lpBuffer : LPSTR , uSize : UINT ,) -> UINT ; pub fn GetSystemWow64DirectoryW (lpBuffer : LPWSTR , uSize : UINT ,) -> UINT ; pub fn IsWow64Process2 (hProcess : HANDLE , pProcessMachine : PUSHORT , pNativeMachine : PUSHORT ,) -> BOOL ; }
};
}

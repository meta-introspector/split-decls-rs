// Generated macro for other_28717 (other)
macro_rules! Depcrate_um_handleapiother_28717 {
() => {
// Module: crate::um::handleapi
// Provides: {"other_28717"}
// Dependencies: {}
extern "system" { pub fn CloseHandle (hObject : HANDLE ,) -> BOOL ; pub fn DuplicateHandle (hSourceProcessHandle : HANDLE , hSourceHandle : HANDLE , hTargetProcessHandle : HANDLE , lpTargetHandle : LPHANDLE , dwDesiredAccess : DWORD , bInheritHandle : BOOL , dwOptions : DWORD ,) -> BOOL ; pub fn CompareObjectHandles (hFirstObjectHandle : HANDLE , hSecondObjectHandle : HANDLE ,) -> BOOL ; pub fn GetHandleInformation (hObject : HANDLE , lpdwFlags : LPDWORD ,) -> BOOL ; pub fn SetHandleInformation (hObject : HANDLE , dwMask : DWORD , dwFlags : DWORD ,) -> BOOL ; }
};
}

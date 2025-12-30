// Generated macro for other_40143 (other)
macro_rules! Depcrate_um_winbaseother_40143 {
() => {
// Module: crate::um::winbase
// Provides: {"other_40143"}
// Dependencies: {}
extern "system" { pub fn CopyFileExA (lpExistingFileName : LPCSTR , lpNewFileName : LPCSTR , lpProgressRoutine : LPPROGRESS_ROUTINE , lpData : LPVOID , pbCancel : LPBOOL , dwCopyFlags : DWORD ,) -> BOOL ; pub fn CopyFileExW (lpExistingFileName : LPCWSTR , lpNewFileName : LPCWSTR , lpProgressRoutine : LPPROGRESS_ROUTINE , lpData : LPVOID , pbCancel : LPBOOL , dwCopyFlags : DWORD ,) -> BOOL ; pub fn CopyFileTransactedA (lpExistingFileName : LPCWSTR , lpNewFileName : LPCWSTR , lpProgressRoutine : LPPROGRESS_ROUTINE , lpData : LPVOID , pbCancel : LPBOOL , dwCopyFlags : DWORD , hTransaction : HANDLE ,) -> BOOL ; pub fn CopyFileTransactedW (lpExistingFileName : LPCWSTR , lpNewFileName : LPCWSTR , lpProgressRoutine : LPPROGRESS_ROUTINE , lpData : LPVOID , pbCancel : LPBOOL , dwCopyFlags : DWORD , hTransaction : HANDLE ,) -> BOOL ; }
};
}

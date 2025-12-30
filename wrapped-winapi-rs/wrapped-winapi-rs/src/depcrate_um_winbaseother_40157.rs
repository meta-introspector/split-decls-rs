// Generated macro for other_40157 (other)
macro_rules! Depcrate_um_winbaseother_40157 {
() => {
// Module: crate::um::winbase
// Provides: {"other_40157"}
// Dependencies: {}
extern "system" { pub fn CopyFile2 (pwszExistingFileName : PCWSTR , pwszNewFileName : PCWSTR , pExtendedParameters : * mut COPYFILE2_EXTENDED_PARAMETERS ,) -> HRESULT ; pub fn MoveFileA (lpExistingFileName : LPCSTR , lpNewFileName : LPCSTR ,) -> BOOL ; pub fn MoveFileW (lpExistingFileName : LPCWSTR , lpNewFileName : LPCWSTR ,) -> BOOL ; pub fn MoveFileExA (lpExistingFileName : LPCSTR , lpNewFileName : LPCSTR , dwFlags : DWORD ,) -> BOOL ; pub fn MoveFileExW (lpExistingFileName : LPCWSTR , lpNewFileName : LPCWSTR , dwFlags : DWORD ,) -> BOOL ; pub fn MoveFileWithProgressA (lpExistingFileName : LPCSTR , lpNewFileName : LPCSTR , lpProgressRoutine : LPPROGRESS_ROUTINE , lpData : LPVOID , dwFlags : DWORD ,) -> BOOL ; pub fn MoveFileWithProgressW (lpExistingFileName : LPCWSTR , lpNewFileName : LPCWSTR , lpProgressRoutine : LPPROGRESS_ROUTINE , lpData : LPVOID , dwFlags : DWORD ,) -> BOOL ; pub fn MoveFileTransactedA (lpExistingFileName : LPCSTR , lpNewFileName : LPCSTR , lpProgressRoutine : LPPROGRESS_ROUTINE , lpData : LPVOID , dwFlags : DWORD , hTransaction : HANDLE ,) -> BOOL ; pub fn MoveFileTransactedW (lpExistingFileName : LPCWSTR , lpNewFileName : LPCWSTR , lpProgressRoutine : LPPROGRESS_ROUTINE , lpData : LPVOID , dwFlags : DWORD , hTransaction : HANDLE ,) -> BOOL ; }
};
}

// Generated macro for other_28656 (other)
macro_rules! Depcrate_um_fileapiother_28656 {
() => {
// Module: crate::um::fileapi
// Provides: {"other_28656"}
// Dependencies: {}
extern "system" { pub fn FindFirstStreamW (lpFileName : LPCWSTR , InfoLevel : STREAM_INFO_LEVELS , lpFindStreamData : LPVOID , dwFlags : DWORD ,) -> HANDLE ; pub fn FindNextStreamW (hFindStream : HANDLE , lpFindStreamData : LPVOID ,) -> BOOL ; pub fn AreFileApisANSI () -> BOOL ; pub fn GetTempPathA (nBufferLength : DWORD , lpBuffer : LPSTR ,) -> DWORD ; pub fn FindFirstFileNameW (lpFileName : LPCWSTR , dwFlags : DWORD , StringLength : LPDWORD , LinkName : PWSTR ,) -> HANDLE ; pub fn FindNextFileNameW (hFindStream : HANDLE , StringLength : LPDWORD , LinkName : PWSTR ,) -> BOOL ; pub fn GetVolumeInformationA (lpRootPathName : LPCSTR , lpVolumeNameBuffer : LPSTR , nVolumeNameSize : DWORD , lpVolumeSerialNumber : LPDWORD , lpMaximumComponentLength : LPDWORD , lpFileSystemFlags : LPDWORD , lpFileSystemNameBuffer : LPSTR , nFileSystemNameSize : DWORD ,) -> BOOL ; pub fn GetTempFileNameA (lpPathName : LPCSTR , lpPrefixString : LPCSTR , uUnique : UINT , lpTempFileName : LPSTR ,) -> UINT ; pub fn SetFileApisToOEM () ; pub fn SetFileApisToANSI () ; }
};
}

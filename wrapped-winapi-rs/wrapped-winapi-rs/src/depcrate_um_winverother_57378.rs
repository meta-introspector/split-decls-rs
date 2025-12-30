// Generated macro for other_57378 (other)
macro_rules! Depcrate_um_winverother_57378 {
() => {
// Module: crate::um::winver
// Provides: {"other_57378"}
// Dependencies: {}
extern "system" { pub fn GetFileVersionInfoSizeA (lptstrFilename : LPCSTR , lpdwHandle : * mut DWORD ,) -> DWORD ; pub fn GetFileVersionInfoSizeW (lptstrFilename : LPCWSTR , lpdwHandle : * mut DWORD ,) -> DWORD ; pub fn GetFileVersionInfoA (lptstrFilename : LPCSTR , dwHandle : DWORD , dwLen : DWORD , lpData : * mut c_void ,) -> BOOL ; pub fn GetFileVersionInfoW (lptstrFilename : LPCWSTR , dwHandle : DWORD , dwLen : DWORD , lpData : * mut c_void ,) -> BOOL ; pub fn VerQueryValueA (pBlock : LPCVOID , lpSubBlock : LPCSTR , lplpBuffer : & mut LPVOID , puLen : PUINT ,) -> BOOL ; pub fn VerQueryValueW (pBlock : LPCVOID , lpSubBlock : LPCWSTR , lplpBuffer : & mut LPVOID , puLen : PUINT ,) -> BOOL ; pub fn VerLanguageNameA (wLang : DWORD , szLang : LPSTR , cchLang : DWORD ,) -> DWORD ; pub fn VerLanguageNameW (wLang : DWORD , szLang : LPWSTR , cchLang : DWORD ,) -> DWORD ; }
};
}

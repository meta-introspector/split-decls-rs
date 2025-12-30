// Generated macro for other_45898 (other)
macro_rules! Depcrate_um_wingdiother_45898 {
() => {
// Module: crate::um::wingdi
// Provides: {"other_45898"}
// Dependencies: {}
extern "system" { pub fn AddFontResourceExA (lpszFilename : LPCSTR , fl : DWORD , pdv : PVOID ,) -> c_int ; pub fn AddFontResourceExW (lpszFilename : LPCWSTR , fl : DWORD , pdv : PVOID ,) -> c_int ; pub fn RemoveFontResourceExA (name : LPCSTR , fl : DWORD , pdv : PVOID ,) -> BOOL ; pub fn RemoveFontResourceExW (name : LPCWSTR , fl : DWORD , pdv : PVOID ,) -> BOOL ; pub fn AddFontMemResourceEx (pbFont : PVOID , cbSize : DWORD , pdv : PVOID , pcFonts : * mut DWORD ,) -> HANDLE ; pub fn RemoveFontMemResourceEx (h : HANDLE ,) -> BOOL ; }
};
}

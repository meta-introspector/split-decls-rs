// Generated macro for other_44090 (other)
macro_rules! Depcrate_um_wincryptother_44090 {
() => {
// Module: crate::um::wincrypt
// Provides: {"other_44090"}
// Dependencies: {}
extern "system" { pub fn CryptStringToBinaryA (pszString : LPCSTR , cchString : DWORD , dwFlags : DWORD , pbBinary : * mut BYTE , pcbBinary : * mut DWORD , pdwSkip : * mut DWORD , pdwFlags : * mut DWORD ,) -> BOOL ; pub fn CryptStringToBinaryW (pszString : LPCWSTR , cchString : DWORD , dwFlags : DWORD , pbBinary : * mut BYTE , pcbBinary : * mut DWORD , pdwSkip : * mut DWORD , pdwFlags : * mut DWORD ,) -> BOOL ; pub fn CryptBinaryToStringA (pbBinary : * const BYTE , cbBinary : DWORD , dwFlags : DWORD , pszString : LPSTR , pcchString : * mut DWORD ,) -> BOOL ; pub fn CryptBinaryToStringW (pbBinary : * const BYTE , cbBinary : DWORD , dwFlags : DWORD , pszString : LPWSTR , pcchString : * mut DWORD ,) -> BOOL ; }
};
}

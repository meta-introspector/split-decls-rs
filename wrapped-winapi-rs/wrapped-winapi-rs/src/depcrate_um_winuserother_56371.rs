// Generated macro for other_56371 (other)
macro_rules! Depcrate_um_winuserother_56371 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56371"}
// Dependencies: {}
extern "system" { pub fn CreateIcon (hInstance : HINSTANCE , nWidth : c_int , nHeight : c_int , cPlanes : BYTE , cBitsPixel : BYTE , lpbANDbits : * const BYTE , lpbXORbits : * const BYTE ,) -> HICON ; pub fn DestroyIcon (hIcon : HICON ,) -> BOOL ; pub fn LookupIconIdFromDirectory (presbits : PBYTE , fIcon : BOOL ,) -> c_int ; pub fn LookupIconIdFromDirectoryEx (presbits : PBYTE , fIcon : BOOL , cxDesired : c_int , cyDesired : c_int , Flags : UINT ,) -> c_int ; pub fn CreateIconFromResource (presbits : PBYTE , dwResSize : DWORD , fIcon : BOOL , dwVer : DWORD ,) -> HICON ; pub fn CreateIconFromResourceEx (presbits : PBYTE , dwResSize : DWORD , fIcon : BOOL , dwVer : DWORD , cxDesired : c_int , cyDesired : c_int , Flags : UINT ,) -> HICON ; }
};
}

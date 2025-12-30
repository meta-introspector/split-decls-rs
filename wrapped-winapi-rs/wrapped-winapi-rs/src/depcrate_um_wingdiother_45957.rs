// Generated macro for other_45957 (other)
macro_rules! Depcrate_um_wingdiother_45957 {
() => {
// Module: crate::um::wingdi
// Provides: {"other_45957"}
// Dependencies: {}
extern "system" { pub fn CreateDIBSection (hdc : HDC , lpbmi : * const BITMAPINFO , usage : UINT , ppvBits : * mut * mut c_void , hSection : HANDLE , offset : DWORD ,) -> HBITMAP ; pub fn GetDIBColorTable (hdc : HDC , iStart : UINT , cEntries : UINT , prgbq : * mut RGBQUAD ,) -> UINT ; pub fn SetDIBColorTable (hdc : HDC , iStart : UINT , cEntries : UINT , prgbq : * const RGBQUAD ,) -> UINT ; }
};
}

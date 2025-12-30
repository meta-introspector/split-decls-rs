// Generated macro for other_45890 (other)
macro_rules! Depcrate_um_wingdiother_45890 {
() => {
// Module: crate::um::wingdi
// Provides: {"other_45890"}
// Dependencies: {}
extern "system" { pub fn GetFontUnicodeRanges (hdc : HDC , lpgs : LPGLYPHSET ,) -> DWORD ; pub fn GetGlyphIndicesA (hdc : HDC , lpstr : LPCSTR , c : c_int , pgi : LPWORD , fl : DWORD ,) -> DWORD ; pub fn GetGlyphIndicesW (hdc : HDC , lpstr : LPCWSTR , c : c_int , pgi : LPWORD , fl : DWORD ,) -> DWORD ; pub fn GetTextExtentPointI (hdc : HDC , pgiIn : LPWORD , cgi : c_int , psize : LPSIZE ,) -> BOOL ; pub fn GetTextExtentExPointI (hdc : HDC , lpwszString : LPWORD , cwchString : c_int , nMaxExtent : c_int , lpnFit : LPINT , lpnDx : LPINT , lpSize : LPSIZE ,) -> BOOL ; pub fn GetCharWidthI (hdc : HDC , giFirst : UINT , cgi : UINT , pgi : LPWORD , piWidths : LPINT ,) -> BOOL ; pub fn GetCharABCWidthsI (hdc : HDC , giFirst : UINT , cgi : UINT , pgi : LPWORD , pabc : LPABC ,) -> BOOL ; }
};
}

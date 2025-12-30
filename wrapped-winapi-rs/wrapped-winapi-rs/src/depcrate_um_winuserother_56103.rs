// Generated macro for other_56103 (other)
macro_rules! Depcrate_um_winuserother_56103 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56103"}
// Dependencies: {}
extern "system" { pub fn DrawTextA (hdc : HDC , lpchText : LPCSTR , cchText : c_int , lprc : LPRECT , format : UINT ,) -> c_int ; pub fn DrawTextW (hdc : HDC , lpchText : LPCWSTR , cchText : c_int , lprc : LPRECT , format : UINT ,) -> c_int ; pub fn DrawTextExA (hdc : HDC , lpchText : LPCSTR , cchText : c_int , lprc : LPRECT , format : UINT , lpdtp : LPDRAWTEXTPARAMS ,) -> c_int ; pub fn DrawTextExW (hdc : HDC , lpchText : LPCWSTR , cchText : c_int , lprc : LPRECT , format : UINT , lpdtp : LPDRAWTEXTPARAMS ,) -> c_int ; pub fn GrayStringA (hDC : HDC , hBrush : HBRUSH , lpOutputFunc : GRAYSTRINGPROC , lpData : LPARAM , nCount : c_int , X : c_int , Y : c_int , nWidth : c_int , nHeight : c_int ,) -> BOOL ; pub fn GrayStringW (hDC : HDC , hBrush : HBRUSH , lpOutputFunc : GRAYSTRINGPROC , lpData : LPARAM , nCount : c_int , X : c_int , Y : c_int , nWidth : c_int , nHeight : c_int ,) -> BOOL ; }
};
}

// Generated macro for other_23446 (other)
macro_rules! Depcrate_um_commctrlother_23446 {
() => {
// Module: crate::um::commctrl
// Provides: {"other_23446"}
// Dependencies: {}
extern "system" { pub fn LoadIconMetric (hinst : HINSTANCE , pszName : PCWSTR , lims : c_int , phico : * mut HICON ,) -> HRESULT ; pub fn LoadIconWithScaleDown (hinst : HINSTANCE , pszName : PCWSTR , cx : c_int , cy : c_int , phico : * mut HICON ,) -> HRESULT ; pub fn DrawShadowText (hdc : HDC , pszText : LPCWSTR , cch : UINT , prc : * mut RECT , dwFlags : DWORD , crText : COLORREF , crShadow : COLORREF , ixOffset : c_int , iyOffset : c_int ,) -> c_int ; }
};
}

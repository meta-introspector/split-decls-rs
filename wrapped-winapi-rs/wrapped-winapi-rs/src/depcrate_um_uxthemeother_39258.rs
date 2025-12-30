// Generated macro for other_39258 (other)
macro_rules! Depcrate_um_uxthemeother_39258 {
() => {
// Module: crate::um::uxtheme
// Provides: {"other_39258"}
// Dependencies: {}
extern "system" { pub fn GetThemeTimingFunction (hTheme : HTHEME , iTimingFunctionId : c_int , pTimingFunction : * mut TA_TIMINGFUNCTION , cbSize : DWORD , pcbSizeOut : * mut DWORD ,) -> HRESULT ; pub fn OpenThemeData (hwnd : HWND , pszClassList : LPCWSTR ,) -> HTHEME ; }
};
}

// Generated macro for SetWindowThemeNonClientAttributes (function)
macro_rules! Depcrate_um_uxthemeSetWindowThemeNonClientAttributes {
() => {
// Module: crate::um::uxtheme
// Provides: {"SetWindowThemeNonClientAttributes"}
// Dependencies: {}
# [inline] pub unsafe fn SetWindowThemeNonClientAttributes (hwnd : HWND , dwMask : DWORD , dwAttributes : DWORD ,) -> HRESULT { use core :: mem :: { size_of , zeroed } ; let mut wta : WTA_OPTIONS = zeroed () ; wta . dwFlags = dwAttributes ; wta . dwMask = dwMask ; SetWindowThemeAttribute (hwnd , WTA_NONCLIENT , & mut wta as * mut WTA_OPTIONS as * mut c_void , size_of :: < WTA_OPTIONS > () as u32 ,) }
};
}

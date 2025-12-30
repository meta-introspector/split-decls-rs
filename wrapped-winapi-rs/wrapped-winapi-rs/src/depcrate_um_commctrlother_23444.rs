// Generated macro for other_23444 (other)
macro_rules! Depcrate_um_commctrlother_23444 {
() => {
// Module: crate::um::commctrl
// Provides: {"other_23444"}
// Dependencies: {}
extern "system" { pub fn SetWindowSubclass (hWnd : HWND , pfnSubclass : SUBCLASSPROC , uIdSubclass : UINT_PTR , dwRefData : DWORD_PTR ,) -> BOOL ; pub fn GetWindowSubclass (hWnd : HWND , pfnSubclass : SUBCLASSPROC , uIdSubclass : UINT_PTR , pdwRefData : * mut DWORD_PTR ,) -> BOOL ; pub fn RemoveWindowSubclass (hWnd : HWND , pfnSubclass : SUBCLASSPROC , uIdSubclass : UINT_PTR ,) -> BOOL ; pub fn DefSubclassProc (hWnd : HWND , uMsg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> LRESULT ; }
};
}

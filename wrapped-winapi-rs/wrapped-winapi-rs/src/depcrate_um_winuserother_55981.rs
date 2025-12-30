// Generated macro for other_55981 (other)
macro_rules! Depcrate_um_winuserother_55981 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55981"}
// Dependencies: {}
extern "system" { pub fn GetSystemMetrics (nIndex : c_int ,) -> c_int ; pub fn GetSystemMetricsForDpi (nIndex : c_int , dpi : UINT ,) -> c_int ; pub fn LoadMenuA (hInstance : HINSTANCE , lpMenuName : LPCSTR ,) -> HMENU ; pub fn LoadMenuW (hInstance : HINSTANCE , lpMenuName : LPCWSTR ,) -> HMENU ; pub fn LoadMenuIndirectA (lpMenuTemplate : * const MENUTEMPLATEA ,) -> HMENU ; pub fn LoadMenuIndirectW (lpMenuTemplate : * const MENUTEMPLATEW ,) -> HMENU ; pub fn GetMenu (hWnd : HWND ,) -> HMENU ; pub fn SetMenu (hWnd : HWND , hMenu : HMENU ,) -> BOOL ; pub fn ChangeMenuA (hMenu : HMENU , cmd : UINT , lpszNewItem : LPCSTR , cmdInsert : UINT , flags : UINT ,) -> BOOL ; pub fn ChangeMenuW (hMenu : HMENU , cmd : UINT , lpszNewItem : LPCWSTR , cmdInsert : UINT , flags : UINT ,) -> BOOL ; pub fn HiliteMenuItem (hWnd : HWND , hMenu : HMENU , uIDHiliteItem : UINT , uHilite : UINT ,) -> BOOL ; pub fn GetMenuStringA (hMenu : HMENU , uIDItem : UINT , lpString : LPSTR , cchMax : c_int , flags : UINT ,) -> c_int ; pub fn GetMenuStringW (hMenu : HMENU , uIDItem : UINT , lpString : LPWSTR , cchMax : c_int , flags : UINT ,) -> c_int ; pub fn GetMenuState (hMenu : HMENU , uId : UINT , uFlags : UINT ,) -> UINT ; pub fn DrawMenuBar (hwnd : HWND ,) -> BOOL ; }
};
}

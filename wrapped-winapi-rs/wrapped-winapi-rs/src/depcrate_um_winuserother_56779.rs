// Generated macro for other_56779 (other)
macro_rules! Depcrate_um_winuserother_56779 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56779"}
// Dependencies: {}
extern "system" { pub fn DefFrameProcA (hwnd : HWND , hwndMDIClient : HWND , uMsg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> LRESULT ; pub fn DefFrameProcW (hwnd : HWND , hwndMDIClient : HWND , uMsg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> LRESULT ; pub fn DefMDIChildProcA (hwnd : HWND , uMsg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> LRESULT ; pub fn DefMDIChildProcW (hwnd : HWND , uMsg : UINT , wParam : WPARAM , lParam : LPARAM ,) -> LRESULT ; pub fn ArrangeIconicWindows (hWnd : HWND ,) -> UINT ; pub fn CreateMDIWindowA (lpClassName : LPCSTR , lpWindowName : LPCSTR , dwStyle : DWORD , X : c_int , Y : c_int , nWidth : c_int , nHeight : c_int , hWndParent : HWND , hInstance : HINSTANCE , lParam : LPARAM ,) -> HWND ; pub fn CreateMDIWindowW (lpClassName : LPCWSTR , lpWindowName : LPCWSTR , dwStyle : DWORD , X : c_int , Y : c_int , nWidth : c_int , nHeight : c_int , hWndParent : HWND , hInstance : HINSTANCE , lParam : LPARAM ,) -> HWND ; pub fn CascadeWindows (hwndParent : HWND , wHow : UINT , lpRect : * const RECT , cKids : UINT , lpKids : * const HWND ,) -> WORD ; }
};
}

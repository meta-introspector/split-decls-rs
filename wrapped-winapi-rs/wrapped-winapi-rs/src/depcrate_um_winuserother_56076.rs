// Generated macro for other_56076 (other)
macro_rules! Depcrate_um_winuserother_56076 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56076"}
// Dependencies: {}
extern "system" { pub fn DragObject (hwndParent : HWND , hwndFrom : HWND , fmt : UINT , data : ULONG_PTR , hcur : HCURSOR ,) -> DWORD ; pub fn DragDetect (hwnd : HWND , pt : POINT ,) -> BOOL ; pub fn DrawIcon (hDC : HDC , x : c_int , y : c_int , hIcon : HICON ,) -> BOOL ; }
};
}

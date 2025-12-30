// Generated macro for other_56267 (other)
macro_rules! Depcrate_um_winuserother_56267 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56267"}
// Dependencies: {}
extern "system" { pub fn GetClassWord (hWnd : HWND , nIndex : c_int ,) -> WORD ; pub fn SetClassWord (hWnd : HWND , nIndex : c_int , wNewWord : WORD ,) -> WORD ; pub fn GetClassLongA (hWnd : HWND , nIndex : c_int ,) -> DWORD ; pub fn GetClassLongW (hWnd : HWND , nIndex : c_int ,) -> DWORD ; pub fn SetClassLongA (hWnd : HWND , nIndex : c_int , dwNewLong : LONG ,) -> DWORD ; pub fn SetClassLongW (hWnd : HWND , nIndex : c_int , dwNewLong : LONG ,) -> DWORD ; # [cfg (target_pointer_width = "64")] pub fn GetClassLongPtrA (hWnd : HWND , nIndex : c_int ,) -> ULONG_PTR ; # [cfg (target_pointer_width = "64")] pub fn GetClassLongPtrW (hWnd : HWND , nIndex : c_int ,) -> ULONG_PTR ; # [cfg (target_pointer_width = "64")] pub fn SetClassLongPtrA (hWnd : HWND , nIndex : c_int , dwNewLong : LONG_PTR ,) -> ULONG_PTR ; # [cfg (target_pointer_width = "64")] pub fn SetClassLongPtrW (hWnd : HWND , nIndex : c_int , dwNewLong : LONG_PTR ,) -> ULONG_PTR ; }
};
}

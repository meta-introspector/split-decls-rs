// Generated macro for other_56147 (other)
macro_rules! Depcrate_um_winuserother_56147 {
() => {
// Module: crate::um::winuser
// Provides: {"other_56147"}
// Dependencies: {}
extern "system" { pub fn LockWindowUpdate (hWndLock : HWND ,) -> BOOL ; pub fn ScrollWindow (hWnd : HWND , xAmount : c_int , yAmount : c_int , lpRect : * const RECT , lpClipRect : * const RECT ,) -> BOOL ; pub fn ScrollDC (hDC : HDC , dx : c_int , dy : c_int , lprcScroll : * const RECT , lprcClip : * const RECT , hrgnUpdate : HRGN , lprcUpdate : LPRECT ,) -> BOOL ; pub fn ScrollWindowEx (hWnd : HWND , dx : c_int , dy : c_int , prcScroll : * const RECT , prcClip : * const RECT , hrgnUpdate : HRGN , prcUpdate : LPRECT , flags : UINT ,) -> c_int ; }
};
}

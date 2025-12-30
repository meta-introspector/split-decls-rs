// Generated macro for GET_XBUTTON_WPARAM (function)
macro_rules! Depcrate_um_winuserGET_XBUTTON_WPARAM {
() => {
// Module: crate::um::winuser
// Provides: {"GET_XBUTTON_WPARAM"}
// Dependencies: {}
# [inline] pub fn GET_XBUTTON_WPARAM (wParam : WPARAM) -> WORD { HIWORD (wParam as DWORD) }
};
}

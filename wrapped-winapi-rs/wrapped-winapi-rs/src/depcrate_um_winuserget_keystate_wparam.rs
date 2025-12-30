// Generated macro for GET_KEYSTATE_WPARAM (function)
macro_rules! Depcrate_um_winuserGET_KEYSTATE_WPARAM {
() => {
// Module: crate::um::winuser
// Provides: {"GET_KEYSTATE_WPARAM"}
// Dependencies: {}
# [inline] pub fn GET_KEYSTATE_WPARAM (wParam : WPARAM) -> WORD { LOWORD (wParam as DWORD) }
};
}

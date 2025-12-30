// Generated macro for GET_NCHITTEST_WPARAM (function)
macro_rules! Depcrate_um_winuserGET_NCHITTEST_WPARAM {
() => {
// Module: crate::um::winuser
// Provides: {"GET_NCHITTEST_WPARAM"}
// Dependencies: {}
# [inline] pub fn GET_NCHITTEST_WPARAM (wParam : WPARAM) -> c_short { LOWORD (wParam as DWORD) as c_short }
};
}

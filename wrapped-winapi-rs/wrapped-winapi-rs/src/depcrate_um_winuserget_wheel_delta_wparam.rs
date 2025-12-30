// Generated macro for GET_WHEEL_DELTA_WPARAM (function)
macro_rules! Depcrate_um_winuserGET_WHEEL_DELTA_WPARAM {
() => {
// Module: crate::um::winuser
// Provides: {"GET_WHEEL_DELTA_WPARAM"}
// Dependencies: {}
# [inline] pub fn GET_WHEEL_DELTA_WPARAM (wParam : WPARAM) -> c_short { HIWORD (wParam as DWORD) as c_short }
};
}

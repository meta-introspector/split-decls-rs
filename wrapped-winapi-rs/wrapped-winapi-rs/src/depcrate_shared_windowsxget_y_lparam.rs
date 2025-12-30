// Generated macro for GET_Y_LPARAM (function)
macro_rules! Depcrate_shared_windowsxGET_Y_LPARAM {
() => {
// Module: crate::shared::windowsx
// Provides: {"GET_Y_LPARAM"}
// Dependencies: {}
# [inline] pub fn GET_Y_LPARAM (lp : LPARAM) -> c_int { HIWORD (lp as DWORD) as c_short as c_int }
};
}

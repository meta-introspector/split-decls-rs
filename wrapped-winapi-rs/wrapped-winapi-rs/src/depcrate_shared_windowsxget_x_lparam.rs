// Generated macro for GET_X_LPARAM (function)
macro_rules! Depcrate_shared_windowsxGET_X_LPARAM {
() => {
// Module: crate::shared::windowsx
// Provides: {"GET_X_LPARAM"}
// Dependencies: {}
# [inline] pub fn GET_X_LPARAM (lp : LPARAM) -> c_int { LOWORD (lp as DWORD) as c_short as c_int }
};
}

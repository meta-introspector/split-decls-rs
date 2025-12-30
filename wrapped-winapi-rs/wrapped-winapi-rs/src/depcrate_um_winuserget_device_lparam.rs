// Generated macro for GET_DEVICE_LPARAM (function)
macro_rules! Depcrate_um_winuserGET_DEVICE_LPARAM {
() => {
// Module: crate::um::winuser
// Provides: {"GET_DEVICE_LPARAM"}
// Dependencies: {}
# [inline] pub fn GET_DEVICE_LPARAM (lParam : LPARAM) -> WORD { HIWORD (lParam as DWORD) & FAPPCOMMAND_MASK }
};
}

// Generated macro for GET_APPCOMMAND_LPARAM (function)
macro_rules! Depcrate_um_winuserGET_APPCOMMAND_LPARAM {
() => {
// Module: crate::um::winuser
// Provides: {"GET_APPCOMMAND_LPARAM"}
// Dependencies: {}
# [inline] pub fn GET_APPCOMMAND_LPARAM (lParam : LPARAM) -> c_short { (HIWORD (lParam as DWORD) & ! FAPPCOMMAND_MASK) as c_short }
};
}

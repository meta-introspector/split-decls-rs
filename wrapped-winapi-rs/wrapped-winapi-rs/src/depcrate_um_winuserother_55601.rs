// Generated macro for other_55601 (other)
macro_rules! Depcrate_um_winuserother_55601 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55601"}
// Dependencies: {}
extern "system" { pub fn RegisterHotKey (hwnd : HWND , id : c_int , fsModifiers : UINT , vk : UINT ,) -> BOOL ; pub fn UnregisterHotKey (hWnd : HWND , id : c_int ,) -> BOOL ; }
};
}

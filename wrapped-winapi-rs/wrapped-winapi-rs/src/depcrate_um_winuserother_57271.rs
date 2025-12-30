// Generated macro for other_57271 (other)
macro_rules! Depcrate_um_winuserother_57271 {
() => {
// Module: crate::um::winuser
// Provides: {"other_57271"}
// Dependencies: {}
extern "system" { pub fn GetAltTabInfoA (hwnd : HWND , iItem : c_int , pati : PALTTABINFO , pszItemText : LPSTR , cchItemText : UINT ,) -> BOOL ; pub fn GetAltTabInfoW (hwnd : HWND , iItem : c_int , pati : PALTTABINFO , pszItemText : LPWSTR , cchItemText : UINT ,) -> BOOL ; pub fn GetListBoxInfo (hwnd : HWND ,) -> DWORD ; pub fn LockWorkStation () -> BOOL ; pub fn UserHandleGrantAccess (hUserHandle : HANDLE , hJob : HANDLE , bGrant : BOOL ,) -> BOOL ; }
};
}

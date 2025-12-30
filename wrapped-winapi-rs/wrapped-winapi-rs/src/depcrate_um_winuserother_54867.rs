// Generated macro for other_54867 (other)
macro_rules! Depcrate_um_winuserother_54867 {
() => {
// Module: crate::um::winuser
// Provides: {"other_54867"}
// Dependencies: {}
extern "system" { pub fn LoadKeyboardLayoutA (pwszKLID : LPCSTR , Flags : DWORD ,) -> HKL ; pub fn LoadKeyboardLayoutW (pwszKLID : LPCWSTR , Flags : DWORD ,) -> HKL ; pub fn ActivateKeyboardLayout (hkl : HKL , Flags : UINT ,) -> HKL ; pub fn ToUnicodeEx (wVirtKey : UINT , wScanCode : UINT , lpKeyState : * const BYTE , pwszBuff : LPWSTR , cchBuff : c_int , wFlags : UINT , dwhkl : HKL ,) -> c_int ; pub fn UnloadKeyboardLayout (hkl : HKL ,) -> BOOL ; pub fn GetKeyboardLayoutNameA (pwszKLID : LPSTR ,) -> BOOL ; pub fn GetKeyboardLayoutNameW (pwszKLID : LPWSTR ,) -> BOOL ; pub fn GetKeyboardLayoutList (nBuff : c_int , lpList : * mut HKL ,) -> c_int ; pub fn GetKeyboardLayout (idThread : DWORD ,) -> HKL ; }
};
}

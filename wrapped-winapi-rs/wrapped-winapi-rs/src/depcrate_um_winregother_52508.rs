// Generated macro for other_52508 (other)
macro_rules! Depcrate_um_winregother_52508 {
() => {
// Module: crate::um::winreg
// Provides: {"other_52508"}
// Dependencies: {}
extern "system" { pub fn InitiateShutdownA (lpMachineName : LPSTR , lpMessage : LPSTR , dwGracePeriod : DWORD , dwShutdownFlags : DWORD , dwReason : DWORD ,) -> DWORD ; pub fn InitiateShutdownW (lpMachineName : LPWSTR , lpMessage : LPWSTR , dwGracePeriod : DWORD , dwShutdownFlags : DWORD , dwReason : DWORD ,) -> DWORD ; pub fn CheckForHiberboot (pHiberboot : PBOOLEAN , bClearFlag : BOOLEAN ,) -> DWORD ; pub fn RegSaveKeyExA (hKey : HKEY , lpFile : LPCSTR , lpSecurityAttributes : LPSECURITY_ATTRIBUTES , Flags : DWORD ,) -> LSTATUS ; pub fn RegSaveKeyExW (hKey : HKEY , lpFile : LPCWSTR , lpSecurityAttributes : LPSECURITY_ATTRIBUTES , Flags : DWORD ,) -> LSTATUS ; }
};
}

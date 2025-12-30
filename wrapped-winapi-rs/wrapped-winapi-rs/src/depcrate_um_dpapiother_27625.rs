// Generated macro for other_27625 (other)
macro_rules! Depcrate_um_dpapiother_27625 {
() => {
// Module: crate::um::dpapi
// Provides: {"other_27625"}
// Dependencies: {}
extern "system" { pub fn CryptProtectMemory (pDataIn : LPVOID , cbDataIn : DWORD , dwFlags : DWORD ,) -> BOOL ; pub fn CryptUnprotectMemory (pDataIn : LPVOID , cbDataIn : DWORD , dwFlags : DWORD ,) -> BOOL ; }
};
}

// Generated macro for other_53707 (other)
macro_rules! Depcrate_um_winspoolother_53707 {
() => {
// Module: crate::um::winspool
// Provides: {"other_53707"}
// Dependencies: {}
extern "system" { pub fn EnumPrintersA (Flags : DWORD , Name : LPSTR , Level : DWORD , pPrinterEnum : LPBYTE , cbBuf : DWORD , pcbNeeded : LPDWORD , pcReturned : LPDWORD ,) -> BOOL ; pub fn EnumPrintersW (Flags : DWORD , Name : LPWSTR , Level : DWORD , pPrinterEnum : LPBYTE , cbBuf : DWORD , pcbNeeded : LPDWORD , pcReturned : LPDWORD ,) -> BOOL ; }
};
}

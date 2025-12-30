// Generated macro for other_52601 (other)
macro_rules! Depcrate_um_winscardother_52601 {
() => {
// Module: crate::um::winscard
// Provides: {"other_52601"}
// Dependencies: {}
extern "system" { pub fn SCardEstablishContext (dwScope : DWORD , pvReserved1 : LPCVOID , pvReserved2 : LPCVOID , phContext : LPSCARDCONTEXT ,) -> LONG ; pub fn SCardReleaseContext (hContext : SCARDCONTEXT ,) -> LONG ; pub fn SCardIsValidContext (hContext : SCARDCONTEXT ,) -> LONG ; }
};
}

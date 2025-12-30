// Generated macro for other_40732 (other)
macro_rules! Depcrate_um_winconother_40732 {
() => {
// Module: crate::um::wincon
// Provides: {"other_40732"}
// Dependencies: {}
extern "system" { pub fn CreateConsoleScreenBuffer (dwDesiredAccess : DWORD , dwShareMode : DWORD , lpSecurityAttributes : * const SECURITY_ATTRIBUTES , dwFlags : DWORD , lpScreenBufferData : LPVOID ,) -> HANDLE ; pub fn SetConsoleCP (wCodePageID : UINT ,) -> BOOL ; pub fn SetConsoleOutputCP (wCodePageID : UINT ,) -> BOOL ; }
};
}

// Generated macro for other_57067 (other)
macro_rules! Depcrate_um_winuserother_57067 {
() => {
// Module: crate::um::winuser
// Provides: {"other_57067"}
// Dependencies: {}
extern "system" { pub fn SetWinEventHook (eventMin : DWORD , eventMax : DWORD , hmodWinEventProc : HMODULE , pfnWinEventProc : WINEVENTPROC , idProcess : DWORD , idThread : DWORD , dwFlags : DWORD ,) -> HWINEVENTHOOK ; pub fn IsWinEventHookInstalled (event : DWORD ,) -> BOOL ; }
};
}

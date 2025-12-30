// Generated macro for other_29058 (other)
macro_rules! Depcrate_um_immother_29058 {
() => {
// Module: crate::um::imm
// Provides: {"other_29058"}
// Dependencies: {}
extern "system" { pub fn ImmGetContext (hwnd : HWND ,) -> HIMC ; pub fn ImmGetOpenStatus (himc : HIMC ,) -> BOOL ; pub fn ImmSetOpenStatus (himc : HIMC , fopen : BOOL ,) -> BOOL ; pub fn ImmSetCompositionWindow (himc : HIMC , lpCompForm : LPCOMPOSITIONFORM ,) -> BOOL ; pub fn ImmReleaseContext (hwnd : HWND , himc : HIMC ,) -> BOOL ; }
};
}

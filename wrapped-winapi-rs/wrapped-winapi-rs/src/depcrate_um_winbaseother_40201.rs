// Generated macro for other_40201 (other)
macro_rules! Depcrate_um_winbaseother_40201 {
() => {
// Module: crate::um::winbase
// Provides: {"other_40201"}
// Dependencies: {}
extern "system" { pub fn CreateActCtxA (pActCtx : PCACTCTXA ,) -> HANDLE ; pub fn CreateActCtxW (pActCtx : PCACTCTXW ,) -> HANDLE ; pub fn AddRefActCtx (hActCtx : HANDLE ,) ; pub fn ReleaseActCtx (hActCtx : HANDLE ,) ; pub fn ZombifyActCtx (hActCtx : HANDLE ,) -> BOOL ; pub fn ActivateActCtx (hActCtx : HANDLE , lpCookie : * mut ULONG_PTR ,) -> BOOL ; pub fn DeactivateActCtx (dwFlags : DWORD , ulCookie : ULONG_PTR ,) -> BOOL ; pub fn GetCurrentActCtx (lphActCtx : * mut HANDLE ,) -> BOOL ; }
};
}

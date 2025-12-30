// Generated macro for other_44332 (other)
macro_rules! Depcrate_um_winevtother_44332 {
() => {
// Module: crate::um::winevt
// Provides: {"other_44332"}
// Dependencies: {}
extern "system" { pub fn EvtOpenSession (LoginClass : EVT_LOGIN_CLASS , Login : PVOID , Timeout : DWORD , Flags : DWORD ,) -> EVT_HANDLE ; pub fn EvtClose (Object : EVT_HANDLE ,) -> BOOL ; pub fn EvtCancel (Object : EVT_HANDLE ,) -> BOOL ; pub fn EvtGetExtendedStatus (BufferSize : DWORD , Buffer : LPWSTR , BufferUsed : PDWORD ,) -> DWORD ; }
};
}

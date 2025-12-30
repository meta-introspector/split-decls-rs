// Generated macro for other_28616 (other)
macro_rules! Depcrate_um_fibersapiother_28616 {
() => {
// Module: crate::um::fibersapi
// Provides: {"other_28616"}
// Dependencies: {}
extern "system" { pub fn FlsAlloc (lpCallback : PFLS_CALLBACK_FUNCTION ,) -> DWORD ; pub fn FlsGetValue (dwFlsIndex : DWORD ,) -> PVOID ; pub fn FlsSetValue (dwFlsIndex : DWORD , lpFlsData : PVOID ,) -> BOOL ; pub fn FlsFree (dwFlsIndex : DWORD ,) -> BOOL ; pub fn IsThreadAFiber () -> BOOL ; }
};
}

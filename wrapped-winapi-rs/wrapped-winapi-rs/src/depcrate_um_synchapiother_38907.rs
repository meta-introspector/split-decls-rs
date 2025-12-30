// Generated macro for other_38907 (other)
macro_rules! Depcrate_um_synchapiother_38907 {
() => {
// Module: crate::um::synchapi
// Provides: {"other_38907"}
// Dependencies: {}
extern "system" { pub fn OpenWaitableTimerW (dwDesiredAccess : DWORD , bInheritHandle : BOOL , lpTimerName : LPCWSTR ,) -> HANDLE ; pub fn SetWaitableTimerEx (hTimer : HANDLE , lpDueTime : * const LARGE_INTEGER , lPeriod : LONG , pfnCompletionRoutine : PTIMERAPCROUTINE , lpArgToCompletionRoutine : LPVOID , WakeContext : PREASON_CONTEXT , TolerableDelay : ULONG ,) -> BOOL ; pub fn SetWaitableTimer (hTimer : HANDLE , lpDueTime : * const LARGE_INTEGER , lPeriod : LONG , pfnCompletionRoutine : PTIMERAPCROUTINE , lpArgToCompletionRoutine : LPVOID , fResume : BOOL ,) -> BOOL ; pub fn CancelWaitableTimer (hTimer : HANDLE ,) -> BOOL ; }
};
}

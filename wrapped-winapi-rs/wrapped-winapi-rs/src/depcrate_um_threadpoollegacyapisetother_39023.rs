// Generated macro for other_39023 (other)
macro_rules! Depcrate_um_threadpoollegacyapisetother_39023 {
() => {
// Module: crate::um::threadpoollegacyapiset
// Provides: {"other_39023"}
// Dependencies: {}
extern "system" { pub fn QueueUserWorkItem (Function : LPTHREAD_START_ROUTINE , Context : PVOID , Flags : ULONG ,) -> BOOL ; pub fn UnregisterWaitEx (WaitHandle : HANDLE , CompletionEvent : HANDLE ,) -> BOOL ; pub fn CreateTimerQueue () -> HANDLE ; pub fn CreateTimerQueueTimer (phNewTimer : PHANDLE , TimerQueue : HANDLE , Callback : WAITORTIMERCALLBACK , Parameter : PVOID , DueTime : DWORD , Period : DWORD , Flags : ULONG ,) -> BOOL ; pub fn ChangeTimerQueueTimer (TimerQueue : HANDLE , Timer : HANDLE , DueTime : ULONG , Period : ULONG ,) -> BOOL ; pub fn DeleteTimerQueueTimer (TimerQueue : HANDLE , Timer : HANDLE , CompletionEvent : HANDLE ,) -> BOOL ; pub fn DeleteTimerQueueEx (TimerQueue : HANDLE , CompletionEvent : HANDLE ,) -> BOOL ; }
};
}

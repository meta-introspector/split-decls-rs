// Generated macro for other_38904 (other)
macro_rules! Depcrate_um_synchapiother_38904 {
() => {
// Module: crate::um::synchapi
// Provides: {"other_38904"}
// Dependencies: {}
extern "system" { pub fn InitializeConditionVariable (ConditionVariable : PCONDITION_VARIABLE ,) ; pub fn WakeConditionVariable (ConditionVariable : PCONDITION_VARIABLE ,) ; pub fn WakeAllConditionVariable (ConditionVariable : PCONDITION_VARIABLE ,) ; pub fn SleepConditionVariableCS (ConditionVariable : PCONDITION_VARIABLE , CriticalSection : PCRITICAL_SECTION , dwMilliseconds : DWORD ,) -> BOOL ; pub fn SleepConditionVariableSRW (ConditionVariable : PCONDITION_VARIABLE , SRWLock : PSRWLOCK , dwMilliseconds : DWORD , Flags : ULONG ,) -> BOOL ; pub fn SetEvent (hEvent : HANDLE ,) -> BOOL ; pub fn ResetEvent (hEvent : HANDLE ,) -> BOOL ; pub fn ReleaseSemaphore (hSemaphore : HANDLE , lReleaseCount : LONG , lpPreviousCount : LPLONG ,) -> BOOL ; pub fn ReleaseMutex (hMutex : HANDLE ,) -> BOOL ; pub fn WaitForSingleObject (hHandle : HANDLE , dwMilliseconds : DWORD ,) -> DWORD ; pub fn SleepEx (dwMilliseconds : DWORD , bAlertable : BOOL ,) -> DWORD ; pub fn WaitForSingleObjectEx (hHandle : HANDLE , dwMilliseconds : DWORD , bAlertable : BOOL ,) -> DWORD ; pub fn WaitForMultipleObjectsEx (nCount : DWORD , lpHandles : * const HANDLE , bWaitAll : BOOL , dwMilliseconds : DWORD , bAlertable : BOOL ,) -> DWORD ; }
};
}

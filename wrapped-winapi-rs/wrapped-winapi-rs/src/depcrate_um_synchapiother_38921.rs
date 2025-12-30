// Generated macro for other_38921 (other)
macro_rules! Depcrate_um_synchapiother_38921 {
() => {
// Module: crate::um::synchapi
// Provides: {"other_38921"}
// Dependencies: {}
extern "system" { pub fn EnterSynchronizationBarrier (lpBarrier : LPSYNCHRONIZATION_BARRIER , dwFlags : DWORD ,) -> BOOL ; pub fn InitializeSynchronizationBarrier (lpBarrier : LPSYNCHRONIZATION_BARRIER , lTotalThreads : LONG , lSpinCount : LONG ,) -> BOOL ; pub fn DeleteSynchronizationBarrier (lpBarrier : LPSYNCHRONIZATION_BARRIER ,) -> BOOL ; pub fn Sleep (dwMilliseconds : DWORD ,) ; pub fn WaitOnAddress (Address : * mut VOID , CompareAddress : PVOID , AddressSize : SIZE_T , dwMilliseconds : DWORD ,) -> BOOL ; pub fn WakeByAddressSingle (Address : PVOID ,) ; pub fn WakeByAddressAll (Address : PVOID ,) ; pub fn SignalObjectAndWait (hObjectToSignal : HANDLE , hObjectToWaitOn : HANDLE , dwMilliseconds : DWORD , bAlertable : BOOL ,) -> DWORD ; pub fn WaitForMultipleObjects (nCount : DWORD , lpHandles : * const HANDLE , bWaitAll : BOOL , dwMilliseconds : DWORD ,) -> DWORD ; pub fn CreateSemaphoreW (lpSemaphoreAttributes : LPSECURITY_ATTRIBUTES , lInitialCount : LONG , lMaximumCount : LONG , lpName : LPCWSTR ,) -> HANDLE ; pub fn CreateWaitableTimerW (lpTimerAttributes : LPSECURITY_ATTRIBUTES , bManualReset : BOOL , lpTimerName : LPCWSTR ,) -> HANDLE ; }
};
}

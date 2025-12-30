// Generated macro for other_38900 (other)
macro_rules! Depcrate_um_synchapiother_38900 {
() => {
// Module: crate::um::synchapi
// Provides: {"other_38900"}
// Dependencies: {}
extern "system" { pub fn InitOnceInitialize (InitOnce : PINIT_ONCE ,) ; pub fn InitOnceExecuteOnce (InitOnce : PINIT_ONCE , InitFn : PINIT_ONCE_FN , Parameter : PVOID , Context : * mut LPVOID ,) -> BOOL ; pub fn InitOnceBeginInitialize (lpInitOnce : LPINIT_ONCE , dwFlags : DWORD , fPending : PBOOL , lpContext : * mut LPVOID ,) -> BOOL ; pub fn InitOnceComplete (lpInitOnce : LPINIT_ONCE , dwFlags : DWORD , lpContext : LPVOID ,) -> BOOL ; }
};
}

// Generated macro for other_38895 (other)
macro_rules! Depcrate_um_synchapiother_38895 {
() => {
// Module: crate::um::synchapi
// Provides: {"other_38895"}
// Dependencies: {}
extern "system" { pub fn InitializeSRWLock (SRWLock : PSRWLOCK ,) ; pub fn ReleaseSRWLockExclusive (SRWLock : PSRWLOCK ,) ; pub fn ReleaseSRWLockShared (SRWLock : PSRWLOCK ,) ; pub fn AcquireSRWLockExclusive (SRWLock : PSRWLOCK ,) ; pub fn AcquireSRWLockShared (SRWLock : PSRWLOCK ,) ; pub fn TryAcquireSRWLockExclusive (SRWLock : PSRWLOCK ,) -> BOOLEAN ; pub fn TryAcquireSRWLockShared (SRWLock : PSRWLOCK ,) -> BOOLEAN ; pub fn InitializeCriticalSection (lpCriticalSection : LPCRITICAL_SECTION ,) ; pub fn EnterCriticalSection (lpCriticalSection : LPCRITICAL_SECTION ,) ; pub fn LeaveCriticalSection (lpCriticalSection : LPCRITICAL_SECTION ,) ; pub fn InitializeCriticalSectionAndSpinCount (lpCriticalSection : LPCRITICAL_SECTION , dwSpinCount : DWORD ,) -> BOOL ; pub fn InitializeCriticalSectionEx (lpCriticalSection : LPCRITICAL_SECTION , dwSpinCount : DWORD , Flags : DWORD ,) -> BOOL ; pub fn SetCriticalSectionSpinCount (lpCriticalSection : LPCRITICAL_SECTION , dwSpinCount : DWORD ,) -> DWORD ; pub fn TryEnterCriticalSection (lpCriticalSection : LPCRITICAL_SECTION ,) -> BOOL ; pub fn DeleteCriticalSection (lpCriticalSection : LPCRITICAL_SECTION ,) ; }
};
}

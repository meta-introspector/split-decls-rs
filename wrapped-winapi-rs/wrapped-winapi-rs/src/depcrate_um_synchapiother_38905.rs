// Generated macro for other_38905 (other)
macro_rules! Depcrate_um_synchapiother_38905 {
() => {
// Module: crate::um::synchapi
// Provides: {"other_38905"}
// Dependencies: {}
extern "system" { pub fn CreateMutexA (lpMutexAttributes : LPSECURITY_ATTRIBUTES , bInitialOwner : BOOL , lpName : LPCSTR ,) -> HANDLE ; pub fn CreateMutexW (lpMutexAttributes : LPSECURITY_ATTRIBUTES , bInitialOwner : BOOL , lpName : LPCWSTR ,) -> HANDLE ; pub fn OpenMutexW (dwDesiredAccess : DWORD , bInheritHandle : BOOL , lpName : LPCWSTR ,) -> HANDLE ; pub fn CreateEventA (lpEventAttributes : LPSECURITY_ATTRIBUTES , bManualReset : BOOL , bInitialState : BOOL , lpName : LPCSTR ,) -> HANDLE ; pub fn CreateEventW (lpEventAttributes : LPSECURITY_ATTRIBUTES , bManualReset : BOOL , bInitialState : BOOL , lpName : LPCWSTR ,) -> HANDLE ; pub fn OpenEventA (dwDesiredAccess : DWORD , bInheritHandle : BOOL , lpName : LPCSTR ,) -> HANDLE ; pub fn OpenEventW (dwDesiredAccess : DWORD , bInheritHandle : BOOL , lpName : LPCWSTR ,) -> HANDLE ; pub fn OpenSemaphoreW (dwDesiredAccess : DWORD , bInheritHandle : BOOL , lpName : LPCWSTR ,) -> HANDLE ; }
};
}

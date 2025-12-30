// Generated macro for other_38912 (other)
macro_rules! Depcrate_um_synchapiother_38912 {
() => {
// Module: crate::um::synchapi
// Provides: {"other_38912"}
// Dependencies: {}
extern "system" { pub fn CreateEventExA (lpEventAttributes : LPSECURITY_ATTRIBUTES , lpName : LPCSTR , dwFlags : DWORD , dwDesiredAccess : DWORD ,) -> HANDLE ; pub fn CreateEventExW (lpEventAttributes : LPSECURITY_ATTRIBUTES , lpName : LPCWSTR , dwFlags : DWORD , dwDesiredAccess : DWORD ,) -> HANDLE ; pub fn CreateSemaphoreExW (lpSemaphoreAttributes : LPSECURITY_ATTRIBUTES , lInitialCount : LONG , lMaximumCount : LONG , lpName : LPCWSTR , dwFlags : DWORD , dwDesiredAccess : DWORD ,) -> HANDLE ; }
};
}

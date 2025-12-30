// Generated macro for other_28516 (other)
macro_rules! Depcrate_um_errhandlingapiother_28516 {
() => {
// Module: crate::um::errhandlingapi
// Provides: {"other_28516"}
// Dependencies: {}
extern "system" { pub fn RaiseFailFastException (pExceptionRecord : PEXCEPTION_RECORD , pContextRecord : PCONTEXT , dwFlags : DWORD ,) ; pub fn FatalAppExitA (uAction : UINT , lpMessageText : LPCSTR ,) ; pub fn FatalAppExitW (uAction : UINT , lpMessageText : LPCWSTR ,) ; pub fn GetThreadErrorMode () -> DWORD ; pub fn SetThreadErrorMode (dwNewMode : DWORD , lpOldMode : LPDWORD ,) -> BOOL ; }
};
}

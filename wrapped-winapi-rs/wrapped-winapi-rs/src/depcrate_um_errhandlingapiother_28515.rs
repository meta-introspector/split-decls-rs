// Generated macro for other_28515 (other)
macro_rules! Depcrate_um_errhandlingapiother_28515 {
() => {
// Module: crate::um::errhandlingapi
// Provides: {"other_28515"}
// Dependencies: {}
extern "system" { pub fn RaiseException (dwExceptionCode : DWORD , dwExceptionFlags : DWORD , nNumberOfArguments : DWORD , lpArguments : * const ULONG_PTR ,) ; pub fn UnhandledExceptionFilter (ExceptionInfo : * mut EXCEPTION_POINTERS ,) -> LONG ; pub fn SetUnhandledExceptionFilter (lpTopLevelExceptionFilter : LPTOP_LEVEL_EXCEPTION_FILTER ,) -> LPTOP_LEVEL_EXCEPTION_FILTER ; pub fn GetLastError () -> DWORD ; pub fn SetLastError (dwErrCode : DWORD ,) ; pub fn GetErrorMode () -> UINT ; pub fn SetErrorMode (uMode : UINT ,) -> UINT ; pub fn AddVectoredExceptionHandler (First : ULONG , Handler : PVECTORED_EXCEPTION_HANDLER ,) -> PVOID ; pub fn RemoveVectoredExceptionHandler (Handle : PVOID ,) -> ULONG ; pub fn AddVectoredContinueHandler (First : ULONG , Handler : PVECTORED_EXCEPTION_HANDLER ,) -> PVOID ; pub fn RemoveVectoredContinueHandler (Handle : PVOID ,) -> ULONG ; }
};
}

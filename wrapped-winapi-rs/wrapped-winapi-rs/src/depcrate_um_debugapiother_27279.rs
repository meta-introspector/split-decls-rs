// Generated macro for other_27279 (other)
macro_rules! Depcrate_um_debugapiother_27279 {
() => {
// Module: crate::um::debugapi
// Provides: {"other_27279"}
// Dependencies: {}
extern "system" { pub fn IsDebuggerPresent () -> BOOL ; pub fn DebugBreak () ; pub fn OutputDebugStringA (lpOutputString : LPCSTR ,) ; pub fn OutputDebugStringW (lpOutputString : LPCWSTR ,) ; pub fn ContinueDebugEvent (dwProcessId : DWORD , dwThreadId : DWORD , dwContinueStatus : DWORD ,) -> BOOL ; pub fn WaitForDebugEvent (lpDebugEvent : LPDEBUG_EVENT , dwMilliseconds : DWORD ,) -> BOOL ; pub fn DebugActiveProcess (dwProcessId : DWORD ,) -> BOOL ; pub fn DebugActiveProcessStop (dwProcessId : DWORD ,) -> BOOL ; pub fn CheckRemoteDebuggerPresent (hProcess : HANDLE , pbDebuggerPresent : PBOOL ,) -> BOOL ; pub fn WaitForDebugEventEx (lpDebugEvent : LPDEBUG_EVENT , dwMilliseconds : DWORD ,) -> BOOL ; }
};
}
